use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "aoe-data",
    about = "Utility for managing contents of the aoc-data repository"
)]
pub struct Args {
    // A flag, true if used in the command line. Note doc comment will
    // be used for the help message of the flag. The name of the
    // argument will be, by default, based on the name of the field.
    /// Activate debug mode
    #[structopt(short, long)]
    pub debug: bool,

    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    pub verbose: u8,

    /// Input file
    #[structopt(short = "i", long = "input", parse(from_os_str))]
    pub input: PathBuf,

    /// Output file, stdout if not present
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    pub output: Option<PathBuf>,

    /// Filetype that should be used for output [yaml, json, toml]
    #[structopt(long = "otype")]
    pub out_type: Option<String>,
    // /// File name: only required when `out-type` is set to `file`
    // #[structopt(name = "FILE", required_if("out-type", "file"))]
    // pub file_name: Option<String>,
    // #[structopt(subcommand)]
    // pub cmd: Command,
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
