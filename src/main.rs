#![deny(clippy::all)]
#![deny(clippy::pedantic)]

mod players;
mod teams;

use crate::players::*;
use crate::teams::*;

use ::serde::{de::DeserializeOwned, Serialize};

use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::{error::Error, io::BufWriter};

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "aoe-data")]
struct Opt {
    // A flag, true if used in the command line. Note doc comment will
    // be used for the help message of the flag. The name of the
    // argument will be, by default, based on the name of the field.
    /// Activate debug mode
    #[structopt(short, long)]
    debug: bool,

    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,

    /// Players input file
    #[structopt(short = "p", long, parse(from_os_str))]
    player_file: PathBuf,

    /// Teams input file (optional)
    #[structopt(short = "t", long, parse(from_os_str))]
    team_file: Option<PathBuf>,

    /// Output file (Optional)
    #[structopt(short = "o", long, parse(from_os_str))]
    output_file: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:#?}", opt);

    let mut players: Vec<Players> = read_list_from_file(opt.player_file).unwrap();
    // let teams: Vec<Teams> = read_list_from_file(opt.team_file).unwrap();

    players.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    // players.dedup_by(|a, b| a.name.eq_ignore_ascii_case(b.name));

    // println!("deserialize: {:#?}", players);
    // println!("deserialize: {:#?}", teams);

    // TODO: Error handling
    write_list_to_file(opt.output_file, players);
}

fn read_list_from_file<P, T>(path: P) -> Result<T, Box<dyn Error>>
where
    P: AsRef<Path>,
    T: DeserializeOwned,
{
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `T`.
    let list = serde_json::from_reader(reader)?;

    // Return the parsed data list
    Ok(list)
}

// TODO: make generic for yaml and json
fn write_list_to_file<P, S>(path: P, obj: S) -> Result<(), std::io::Error>
where
    P: AsRef<Path>,
    S: Serialize,
{
    // Open the file in writable mode with buffer.
    let file = File::create(path)?;
    let writer = BufWriter::new(file);

    // Write data to file
    serde_yaml::to_writer(writer, &obj).expect("Unable to write data");

    Ok(())
}
