//! Library for managing aoe-reference-data files

#![deny(clippy::all)]
#![deny(clippy::pedantic)]
// Relax compiler warnings
// TODO: Temporary, remove later
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(missing_docs)]
#![allow(dead_code)]
#![allow(unused_variables)]

pub mod cli;
mod core;
mod datalists;

use crate::core::parsing::DataLists;
use crate::datalists::{platforms, players, teams};
use log::{debug, error, info, trace, warn};
use stable_eyre::eyre::{eyre, Report, Result, WrapErr};
use std::ffi::OsStr;
// use structopt::{clap::Arg, StructOpt};

/// Entrypoint for the library part of the Executable's main function
pub fn run(config: cli::Args) -> Result<(), Report> {
    debug!("CLI config: {:#?}", config);
    trace!("We are inside the run-function!");

    let data_list: DataLists;

    // Open and parse files
    match DataLists::new_from_cli_config(config) {
        Err(err) => return Err(err),
        Ok(k) => data_list = k,
    };

    debug!("deserialize: {:#?}", data_list.player_list);
    debug!("deserialize: {:#?}", data_list.team_list);
    debug!("deserialize: {:#?}", data_list.platform_list);

    Ok(())
}

// let mut lists: DataLists;
// let file_type: FileType;

// if let Some(x) = DataLists::get_file_type_from_extension(
//     &opt.input.extension().and_then(OsStr::to_str).unwrap(),
// ) {
//     let file_type = x;
// }
// players.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
// players.dedup_by(|a, b| a.name.eq_ignore_ascii_case(b.name));

//     if let Some(output) = opt.output {
//         write_list_to_file(output, players);
//     }
