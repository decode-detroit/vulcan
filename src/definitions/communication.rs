// Copyright (c) 2019-2021 Decode Detroit
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

//! This module implements shared communication structures for communicating
//! across the modules of the system.

// Import crate definitions
use crate::definitions::*;

// Import Tokio features
use tokio::sync::{mpsc, oneshot};

/// The stucture and methods to send WebRequests to the system interface
///
#[derive(Clone, Debug)]
pub struct WebSend {
    web_send: mpsc::Sender<WebRequest>, // the mpsc sending line to pass web requests
}

// Implement the key features of the web send struct
impl WebSend {
    /// A function to create a new WebSend
    ///
    /// The function returns the the Web Sent structure and the system
    /// receive channel which will return the provided updates.
    ///
    pub fn new() -> (Self, mpsc::Receiver<WebRequest>) {
        // Create the new channel
        let (web_send, receive) = mpsc::channel(256);

        // Create and return both new items
        (WebSend { web_send }, receive)
    }

    /// A method to send a web request. This method fails silently.
    ///
    pub async fn send(&self, reply_to: oneshot::Sender<WebReply>, request: Request) {
        self.web_send
            .send(WebRequest { reply_to, request })
            .await
            .unwrap_or(());
    }
}

/// A structure for carrying requests from the web interface
///
pub struct WebRequest {
    pub reply_to: oneshot::Sender<WebReply>, // the handle for replying to the reqeust
    pub request: Request,                    // the request
}

/// An enum to carry requests
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Request {
    // TODO A variant to stop set all lights to their emergency values
    //AllStop,

    // TODO A variant to define the emergency values of all the lights
    //DefineAllStop {
    //    universe: Universe, // the correct value of all the channels in emergency mode
    //},

    /// A variant to play a fade on a channel
    PlayFade {
        fade: Fade, // the desired fade animation
    },

    /// A variant to restore all the lights to a pre-defined value
    Restore {
        universe: Universe,
    },

    /// A variant to quit the program
    Quit,
}

/// A type to cover all web replies
///
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum WebReply {
    // A variant for replies with no specific content
    #[serde(rename_all = "camelCase")]
    Generic {
        is_valid: bool,  // a flag to indicate the result of the request
        message: String, // a message describing the success or failure
    },
}

// Implement key features of the web reply
impl WebReply {
    /// A function to return a new, successful web reply
    ///
    pub fn success() -> WebReply {
        WebReply::Generic {
            is_valid: true,
            message: "Request completed.".to_string(),
        }
    }

    /// A function to return a new, failed web reply
    ///
    pub fn failure<S>(reason: S) -> WebReply
    where
        S: Into<String>,
    {
        WebReply::Generic {
            is_valid: false,
            message: reason.into(),
        }
    }

    /// A method to check if the reply is a success
    ///
    pub fn is_success(&self) -> bool {
        match self {
            &WebReply::Generic { ref is_valid, .. } => is_valid.clone(),
        }
    }
}
