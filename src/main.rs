//! Executable for managing aoe-reference-data files.
//!
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
// TODO: Temporary, remove later
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(missing_docs)]
#![allow(dead_code)]

// Error handling
#[macro_use]
extern crate log;
use human_panic::setup_panic;
use simple_log::LogConfigBuilder;
use stable_eyre::eyre::{eyre, Report, Result, WrapErr};
use std::process;

// Crate internals
use aoe_data_util::cli::Args;

// CLI
use structopt::StructOpt;

fn main() -> Result<(), Report> {
    // Install the panic and error report handlers
    stable_eyre::install()?;

    // Human Panic. Only enabled when *not* debugging.
    #[cfg(not(debug_assertions))]
    {
        setup_panic!(Metadata {
            name: env!("CARGO_PKG_NAME").into(),
            version: env!("CARGO_PKG_VERSION").into(),
            authors: "Simonsan <simon@systemli.org>".into(),
            homepage: "https://github.com/transparencies/aoe-data-util/issues"
                .into(),
        });
    }

    // Calling the command line parsing logic with the argument values
    let cli_args = aoe_data_util::cli::Args::from_args();

    // If `debug` flag is set, we use a logfile
    if cli_args.debug {
        // Setting up logfile
        let log_setup = LogConfigBuilder::builder()
            .path(&cli_args.log_file_path)
            .size(1 * 100)
            .roll_count(10)
            .level(&cli_args.log_level)
            .output_file()
            .output_console()
            .build();

        simple_log::new(log_setup.clone()).expect("Log setup failed!");
        debug!("Log config: {:?}", &log_setup);
        trace!("Logs were set up.");
    }

    // Setting up any other configuration
    // TODO

    // Calling run function in lib.rs
    // Handling the error if run returns an error
    match aoe_data_util::run(cli_args) {
        Err(e) => Err(e).wrap_err("aoe-data-util experienced a failure!"),
        Ok(k) => Ok(k),
    }
}
