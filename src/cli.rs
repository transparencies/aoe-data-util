//! Everything regarding the commandline interface

// TODO: Temporary, remove later
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(missing_docs)]
#![allow(dead_code)]

use std::path::PathBuf;
use structopt::StructOpt;

/// StructOpt's struct for parsing commandline input
#[derive(StructOpt, Debug)]
#[structopt(
    name = "aoe-data-util",
    about = "Utility for managing contents of the aoc-data repository"
)]
pub struct Args {
    // A flag, true if used in the command line. Note doc comment will
    // be used for the help message of the flag. The name of the
    // argument will be, by default, based on the name of the field.
    /// Activate debug mode
    #[structopt(short, long)]
    pub debug: bool,

    /// Log file path
    #[structopt(long = "log-file", default_value = "./logs/aoe-data-util.log")]
    pub log_file_path: String,

    /// Log file path
    #[structopt(long = "log-level", default_value = "debug")]
    pub log_level: String,

    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    pub verbose: u8,

    /// Input file
    #[structopt(long = "players", parse(from_os_str))]
    pub players_input_path: Option<PathBuf>,

    /// Input file
    #[structopt(long = "teams", parse(from_os_str))]
    pub teams_input_path: Option<PathBuf>,

    /// Input file
    #[structopt(long = "platforms", parse(from_os_str))]
    pub platforms_input_path: Option<PathBuf>,

    /// Output file, stdout if not present
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    pub output_path: Option<PathBuf>,

    /// Filetype that should be used for output [yaml, json, toml]
    #[structopt(long = "otype")]
    pub output_type: Option<String>,
    /* /// File name: only required when `out-type` is set to `file`
     * #[structopt(name = "FILE", required_if("out-type", "file"))]
     * pub file_name: Option<String>,
     * #[structopt(subcommand)]
     * pub cmd: Command, */
}

// #[derive(StructOpt, Debug)]
// pub enum Command {
//     /// Add value to item
//     Add {},
//     /// Edit value of item
//     Edit {},
//     /// Remove value from item
//     Remove {},
// }
