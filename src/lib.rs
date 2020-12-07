//! Library for managing aoe-reference-data files
//!

#![deny(clippy::all)]
#![deny(clippy::pedantic)]
// TODO: Temporary, remove later
#![allow(dead_code)]
#![allow(unused_imports)]
#![warn(missing_docs)]

pub mod cli;
mod core;
mod datalists;

use log::{debug, error, info, trace, warn};
use stable_eyre::eyre::{eyre, Report, Result, WrapErr};
// use structopt::{clap::Arg, StructOpt};

/// Entrypoint for the library part of the Executable's main function
pub fn run(config: cli::Args) -> Result<(), Report> {
    debug!("{:#?}", config);
    debug!("Welcome to the run-function");

    Ok(())
}

// let mut lists: DataLists;
// let file_type: FileType;

// if let Some(x) = DataLists::get_file_type_from_extension(
//     &opt.input.extension().and_then(OsStr::to_str).unwrap(),
// ) {
//     let file_type = x;
// }

// let players: Vec<players::Players> = DataLists::read_list_from_file(&opt.input).unwrap();

// players.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

// players.dedup_by(|a, b| a.name.eq_ignore_ascii_case(b.name));

// println!("deserialize: {:#?}", players);
// println!("deserialize: {:#?}", teams);

//     if let Some(output) = opt.output {
//         write_list_to_file(output, players);
//     }
