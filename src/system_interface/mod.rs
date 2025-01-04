// Copyright (c) 2019-20 Decode Detroit
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

//! A module to create and monitor the user interface and the system inputs.
//! This module links directly to the event handler and sends any updates
//! to the application window.

// Define submodules
mod backup_handler;
mod dmx_interface;

// Import crate definitions
use crate::definitions::*;

// Import submodute definitions
use backup_handler::BackupHandler;
use dmx_interface::DmxInterface;

// Import standard library features
use std::path::PathBuf;

// Import Tokio features
use tokio::sync::mpsc;

// Import anyhow features
use anyhow::Result;

/// A structure to contain the system interface and handle all updates to the
/// to the DMX controller
///
pub struct SystemInterface {
    web_receive: mpsc::Receiver<WebRequest>, // the receiving line for web requests
    dmx_interface: DmxInterface, // the structure for controlling dmx playback
    backup_handler: BackupHandler, // the structure for maintaining the backup
}

// Implement key SystemInterface functionality
impl SystemInterface {
    /// A function to create a new, blank instance of the system interface.
    ///
    pub async fn new(path: PathBuf, address: String, server_location: Option<String>) -> Result<(Self, WebSend)> {
        // Create the web send for the web interface
        let (web_send, web_receive) = WebSend::new();

        // Try to initialize the dmx interface
        let dmx_interface = DmxInterface::new(&path)?;

        // Try to initialize the backup handler
        let mut backup_handler = BackupHandler::new(address, server_location).await;

        // Check for existing data from the backup handler
        if let Some(universe) = backup_handler.reload_backup() {
            // Load the universe onto the dmx hardware
            dmx_interface.set_universe(universe).await;
        }

        // Create the new system interface instance
        let sys_interface = SystemInterface {
            web_receive,
            dmx_interface,
            backup_handler,
        };

        // Regardless, return the new SystemInterface and general send line
        Ok((sys_interface, web_send))
    }

    /// A method to run one iteration of the system interface to update the underlying system of any event changes.
    ///
    async fn run_once(&mut self) -> bool {
        // Check for updates on any line
        tokio::select! {
            // Updates from the Web Interface
            Some(request) = self.web_receive.recv() => {
                // Match the request subtype
                match request.request {
                    // If performing a fade
                    Request::PlayFade { fade } => {
                        // Try to pass new fade to the dmx inferface
                        if let Err(error) = self.dmx_interface.play_fade(fade.clone()).await {
                            request.reply_to.send(WebReply::failure(format!("{}", error))).unwrap_or(());
                        
                        // Otherwise
                        } else {
                            // Save to the backup
                            self.backup_handler.backup_fade(fade).await;

                            // And indicate success
                            request.reply_to.send(WebReply::success()).unwrap_or(());
                        }
                    }

                    // If loading the dmx universe
                    Request::LoadUniverse { universe } => {
                        // Pass the universe settings to the dmx interface
                        self.dmx_interface.set_universe(universe.clone()).await;

                        // Save the universe to the backup
                        self.backup_handler.backup_universe(universe).await;

                        // Reply success to the web interface
                        request.reply_to.send(WebReply::success()).unwrap_or(());
                    }

                    // If closing the program
                    Request::Close => {
                        // End the loop
                        return false;
                    }
                }
            }
        }

        // In most cases, indicate to continue normally
        true
    }

    /// A method to run an infinite number of interations of the system
    /// interface to update the underlying system of any media changes.
    ///
    /// When this loop completes, it will consume the system interface and drop
    /// all associated data.
    ///
    pub async fn run(mut self) {
        // Loop the structure indefinitely
        loop {
            // Repeat endlessly until run_once reaches close
            if !self.run_once().await {
                break;
            }
        }

        // Drop all associated data in system interface
        drop(self);
    }
}
