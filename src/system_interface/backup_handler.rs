// Copyright (c) 2024 Decode Detroit
// Author: Patton Doyle
// Licence: GNU GPLv3
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

//! This module implements the connection to a Redis backup server to maintain
//! a backup of the program state. This handler syncs the current media playlist
//!  to the server. This module does nothing if a Redis server is not connected.
//!
//! WARNING: This module assumes no authorized systems/operators are compromised.

// Import crate definitions
use crate::definitions::*;

// Import tracing features
use tracing::{error, warn};

// Imprt redis client library
use redis::{Commands, ConnectionLike, RedisResult};

// Import YAML processing library
use serde_yaml;

/// A structure which holds a reference to the Redis server (if it exists) and
/// syncronizes local data to and from the server.
///
/// # Notes
///
/// When created, the status handler will attempt to connect to the requested
/// redis server. If the status handler cannot make the connection, the status
/// handler will raise an error and return none.
///
pub struct BackupHandler {
    address: String, // the listening address for this instance of the controller for unique identification
    connection: Option<redis::Connection>, // the Redis connection, if it exists
    universe: Universe, // the current state of all the DMX channels
}

// Implement key features for the status handler
impl BackupHandler {
    /// A function to create and return a new backup handler.
    ///
    /// # Errors
    ///
    /// This function will raise an error if it is unable to connect to the
    /// Redis server provided.
    ///
    pub async fn new(address: String, server_location: Option<String>) -> Self {
        // If a server location was specified
        if let Some(location) = server_location {
            // Try to connect to the Redis server
            if let Ok(client) = redis::Client::open(location.as_str()) {
                // Try to get a copy of the Redis connection
                if let Ok(mut connection) = client.get_connection() {
                    // Set the snapshot settings
                    let result: RedisResult<redis::Value> = connection.req_command(
                        redis::Cmd::new()
                            .arg("CONFIG")
                            .arg("SET")
                            .arg("save")
                            .arg("60 1"),
                    );

                    // Unpack the result from the operation
                    if let Err(..) = result {
                        // Warn that it wasn't possible to update the current scene
                        error!("Unable to set Redis snapshot settings.");
                    }

                    // Return the new backup handler
                    return Self {
                        address,
                        connection: Some(connection),
                        universe: Universe::new(),
                    };

                // Indicate that there was a failure to connect to the server
                } else {
                    error!("Unable to connect to backup server: {}.", location);
                }

            // Indicate that there was a failure to connect to the server
            } else {
                error!("Unable to connect to backup server: {}.", location);
            }
        }

        // If a location was not specified or the connection failed, return without a redis connection
        Self {
            address,
            connection: None,
            universe: Universe::new(),
        }
    }

    /// A method to backup a new fade to the backup server.
    ///
    /// # Errors
    ///
    /// This function will raise an error if it is unable to connect to the
    /// Redis server.
    ///
    pub async fn backup_fade(&mut self, fade: Fade) {
        // If the redis connection exists
        if let Some(mut connection) = self.connection.take() {
            // Add the channel to the current universe
            self.universe.set(fade.channel, fade.value);

            // Try to serialize the universe
            let universe_string = match serde_yaml::to_string(&self.universe) {
                Ok(string) => string,
                Err(error) => {
                    error!("Unable to parse universe: {}.", error);

                    // Put the connection back
                    self.connection = Some(connection);
                    return;
                }
            };

            // Try to copy the data to the server
            let result: RedisResult<bool> = connection.set(
                &format!("vulcan:{}:universe", self.address),
                &universe_string,
            );

            // Alert that the channel list was not set
            if let Err(..) = result {
                error!("Unable to backup fade onto backup server.");
            }

            // Put the connection back
            self.connection = Some(connection);
        }
    }

    /// A method to backup the full universe to the backup server.
    ///
    /// # Errors
    ///
    /// This function will raise an error if it is unable to connect to the
    /// Redis server.
    ///
    pub async fn backup_universe(&mut self, universe: Universe) {
        // If the redis connection exists
        if let Some(mut connection) = self.connection.take() {
            // Replace the current universe
            self.universe = universe;

            // Try to serialize the universe
            let universe_string = match serde_yaml::to_string(&self.universe) {
                Ok(string) => string,
                Err(error) => {
                    error!("Unable to parse universe: {}.", error);

                    // Put the connection back
                    self.connection = Some(connection);
                    return;
                }
            };

            // Try to copy the data to the server
            let result: RedisResult<bool> = connection.set(
                &format!("vulcan:{}:universe", self.address),
                &universe_string,
            );

            // Alert that the channel list was not set
            if let Err(..) = result {
                error!("Unable to backup universe onto backup server.");
            }

            // Put the connection back
            self.connection = Some(connection);
        }
    }

    /// A method to reload an existing backup from the backup server. If the
    /// data exists, this function returns the existing backup data.
    ///
    /// # Errors
    ///
    /// This function will raise an error if it is unable to connect to the
    /// Redis server.
    ///
    pub fn reload_backup(&mut self) -> Option<Universe> {
        // If the redis connection exists
        if let Some(mut connection) = self.connection.take() {
            // Check to see if there is a universe
            let result: RedisResult<String> =
                connection.get(&format!("vulcan:{}:universe", self.address));

            // If something was received
            if let Ok(universe_string) = result {
                // Warn that existing data was found
                warn!("Vulcan detected lingering backup data. Reloading ...");

                // Try to parse the data
                let mut universe = Universe::new();
                if let Ok(new_universe) = serde_yaml::from_str(universe_string.as_str()) {
                    universe = new_universe;
                }

                // Save the universe
                self.universe = universe.clone();

                // Put the connection back
                self.connection = Some(connection);

                // Return all the backup information
                return Some(universe);
            }

            // Put the connection back
            self.connection = Some(connection);
        }

        // Silently return nothing if the connection does not exist or there was not any data
        None
    }
}

// Implement the drop trait for the backup handler struct.
impl Drop for BackupHandler {
    /// This method removes all the the existing data from the server.
    ///
    /// # Errors
    ///
    /// This method will ignore any errors as it is called only when the backup
    /// connection is being closed.
    ///
    fn drop(&mut self) {
        // If the redis connection exists
        if let Some(mut connection) = self.connection.take() {
            // Try to delete the universe backup if it exists
            let _: RedisResult<bool> = connection.del(&format!("vulcan:{}:universe", self.address));
        }
    }
}

// Tests of the status module
#[cfg(test)]
mod tests {
    use super::*;

    // Test the backup module
    #[tokio::test]
    async fn backup_dmx() {
        // Create the backup handler
        let mut backup_handler = BackupHandler::new(
            String::from("127.0.0.1:27655"),
            Some(String::from("redis://127.0.0.1:6379")),
        )
        .await;

        // Make sure there is no existing backup
        if backup_handler.reload_backup().is_some() {
            panic!("Backup already existed before beginning of the test.");
        }

        // Load a universe and several fades to the universe
        let mut universe = Universe::new();
        universe.set(1, 255);
        universe.set(2, 255);
        backup_handler.backup_universe(universe).await;
        backup_handler
            .backup_fade(Fade {
                channel: 2,
                value: 150,
                duration: None,
            })
            .await;
        backup_handler
            .backup_fade(Fade {
                channel: 5,
                value: 255,
                duration: None,
            })
            .await;
        backup_handler
            .backup_fade(Fade {
                channel: 6,
                value: 150,
                duration: None,
            })
            .await;

        // Reload the backup
        if let Some(universe) = backup_handler.reload_backup() {
            assert_eq!(universe.get(1), 255);
            assert_eq!(universe.get(2), 150);
            assert_eq!(universe.get(5), 255);
            assert_eq!(universe.get(6), 150);

        // If the backup doesn't exist, throw the error
        } else {
            panic!("Backup was not reloaded.");
        }
    }
}
