// Copyright (c) 2021 Decode Detroit
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

//! This module implements structures to communicate various mdmx parameters.

// Import standard library features
use std::time::Duration;

/// A struct to define a single fade of a dmx channel.
/// A fade consists of a channel number, a desired final,
/// and a duration of the change between values. All fades
/// are linear (for now).
///
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fade {
    pub channel: u32,               // the dmx channel to fade
    pub value: u8,                  // the final value at the end of the fade
    pub duration: Option<Duration>, // the duration of the fade (None if instantaneous)
}

// Define the DMX constants
pub const DMX_MAX: u32 = 512; // the highest channel of DMX, exclusive

/// A type definition for one set of Dmx channels
///
/// NOTE: the chennels are internally zero-indexed,
/// rather than the one-indexed standard of DMX
///
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Universe {
    values: Vec<u8>, // Internal representation of the channel values
}

/// Implement key features for the DmxUniverse
impl Universe {
    /// Function to create a new, initialized list of the dmx channels
    ///
    pub fn new() -> Self {
        Self {
            values: vec![0; DMX_MAX as usize],
        }
    }

    /// Method to get the value of a particular channel
    ///
    pub fn get(&self, channel: u32) -> u8 {
        // Check the bounds
        if (channel > DMX_MAX) | (channel < 1) {
            return 0; // default to zero
        }

        // Otherwise, convert to zero-indexed and return the value
        return self.values[channel as usize - 1];
    }

    /// Method to set the value of a paticular channel
    ///
    pub fn set(&mut self, channel: u32, value: u8) {
        // Check the bounds
        if (channel <= DMX_MAX) & (channel > 0) {
            // Convert to zero-indexed and set the value
            self.values[channel as usize - 1] = value;
        } // Otherwise, do nothing
    }

    /// Method to export the universe as a set of bytes
    ///
    /// CAUTION: These bytes are zero-indexed!
    ///
    pub fn as_bytes(&self) -> Vec<u8> {
        // Return the array
        self.values.clone()
    }
}
