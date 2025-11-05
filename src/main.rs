// Copyright (c) 2019 Decode Detroit
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

//! The main module of the vulcan program which pulls from the other modules.

// Allow deeper recursion testing for web server
#![recursion_limit = "256"]

// Import YAML processing libraries
#[macro_use]
extern crate serde;

// Define program modules
#[macro_use]
mod definitions;
mod system_interface;
mod web_interface;

// Import crate definitions
use crate::definitions::*;

// Import other structures into this module
use self::system_interface::SystemInterface;
use self::web_interface::WebInterface;

// Import standard library features
use std::path::PathBuf;

// Import anyhow features
#[macro_use]
extern crate anyhow;

// Import tracing features
use tracing_subscriber::filter::LevelFilter;

// Import clap features
use clap::Parser;

/// Struct to hold the optional arguments for Minerva
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    /// Hardware address for the DMX connection
    #[arg(short, long)]
    path: PathBuf,

    /// Address for the web interface
    #[arg(short, long, default_value = DEFAULT_ADDRESS)]
    address: String,

    /// Address for the backup server
    #[arg(short, long, default_value = None)]
    backup: Option<String>,

    /// Flag to set the log level
    #[arg(short, long, default_value = DEFAULT_LOGLEVEL)]
    log_level: String,
}

/// The Vulcan structure to contain the program launching and overall
/// communication code.
///
struct Vulcan;

// Implement the Vulcan functionality
impl Vulcan {
    /// A function to setup the logging configuration
    ///
    fn setup_logging(log_string: String) {
        // Try to convert the string to a log level
        let log_level = match log_string.as_str() {
            "Trace" => LevelFilter::TRACE,
            "Debug" => LevelFilter::DEBUG,
            "Info" => LevelFilter::INFO,
            "Warn" => LevelFilter::WARN,
            "Error" => LevelFilter::ERROR,

            // Otherwise, print a nice error
            _ => {
                println!(
                    "Unable to parse parameter for option 'logLevel'. Options are Trace, Debug, Info, Warn, and Error."
                );
                LevelFilter::INFO
            }
        };

        // Initialize tracing
        tracing_subscriber::fmt()
            .with_max_level(log_level)
            .with_target(false)
            .init();
    }

    /// A function to build the main program and the web interface
    ///
    async fn run(arguments: Arguments) {
        // Initialize logging (guard is held until the end of run())
        let _guard = Vulcan::setup_logging(arguments.log_level);

        // Launch the system interface to connect and control the DMX signals
        let (system_interface, web_send) =
            SystemInterface::new(arguments.path, arguments.address.clone(), arguments.backup)
                .await
                .expect("Unable to create the System Interface.");

        // Create the web interface
        let mut web_interface = WebInterface::new(web_send, arguments.address);

        // Run the web interface in a new thread
        tokio::spawn(async move {
            web_interface.run().await;
        });

        // Block on the system interface
        system_interface.run().await;
    }
}

/// The main function of the program, simplified to as high a level as possible.
///
#[tokio::main]
async fn main() {
    // Get the commandline arguments
    let arguments = Arguments::parse();

    // Create the program and run until directed otherwise
    Vulcan::run(arguments).await;
}
