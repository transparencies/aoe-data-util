#![deny(clippy::all)]
#![deny(clippy::pedantic)]

mod cli;
mod platforms;
mod players;
mod teams;

use crate::cli::*;
use crate::platforms::*;
use crate::players::*;
use crate::teams::*;

use ::serde::{de::DeserializeOwned, Serialize};

use std::ffi::OsStr;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::path::Path;
use std::{error::Error, path::PathBuf};

use structopt::{clap::Arg, StructOpt};

#[derive(Debug)]
pub enum FileType {
    Json,
    Toml,
    Yaml,
}

impl FileType {
    fn get_file_type_from_extension(ext: &str) -> Option<FileType> {
        match ext {
            "yaml" | "yml" => Some(FileType::Yaml),
            "json" => Some(FileType::Json),
            "toml" => Some(FileType::Toml),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
struct DataLists {
    player_list: Vec<players::Players>,
    teams_list: Vec<teams::Teams>,
    platforms_list: Vec<platforms::Platforms>,
}

impl DataLists {
    // fn get_file_name_from_path(path: AsRef<Path>) -> Option<&str> {
    //     path.file_name().and_then(OsStr::to_str?)
    // }

    // fn load_data_from_input_file(file_type: FileType, path: &dyn AsRef<Path>) -> Datalists {
    //     match filename {
    //         "players" => {
    //             let lists = DataLists {
    //                 player_list: Some(
    //                     read_list_from_file::<players::Players>(&opt.input, file_type)
    //                         .expect("Parsing of players file failed."),
    //                 ),
    //                 teams_list: None,
    //                 platforms_list: None,
    //             };
    //         }
    //         "teams" => {
    //             let lists = DataLists {
    //                 teams_list: Some(
    //                     read_list_from_file::<teams::Teams>(&opt.input, file_type)
    //                         .expect("Parsing of teams file failed."),
    //                 ),
    //                 player_list: None,
    //                 platforms_list: None,
    //             };
    //         }
    //         "platforms" => {
    //             let lists = DataLists {
    //                 platforms_list: Some(
    //                     read_list_from_file::<platforms::Platforms>(&opt.input, file_type)
    //                         .expect("Parsing of platforms file failed.of"),
    //                 ),
    //                 player_list: None,
    //                 teams_list: None,
    //             };
    //         }
    //         _ => {
    //             println!("File type of input file doesn't match to any deserializable format.");
    //             panic!();
    //         }
    //     }
    // }

    fn read_list_from_file<T>(
        path: &dyn AsRef<Path>,
        ftype: Option<FileType>,
    ) -> Result<Vec<T>, Box<dyn Error>>
    where
        T: DeserializeOwned,
    {
        // Open the file in read-only mode with buffer.
        let file = File::open(&path)?;
        let reader = BufReader::new(file);

        match ftype {
            Some(FileType::Yaml) => {
                // Read the YAML contents of the file as an instance of `Vec<T>`.
                let list: Vec<T> = serde_yaml::from_reader(reader)?;
                Ok(list)
            }
            Some(FileType::Json) => {
                // Read the JSON contents of the file as an instance of `Vec<T>`.
                let list: Vec<T> = serde_json::from_reader(reader)?;
                Ok(list)
            }
            _ => {
                println!("File type of input file doesn't match to any deserializable format.");
                panic!();
            }
        }
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
}

fn main() {
    let opt = Args::from_args();
    println!("{:#?}", opt);

    // if let Some(output) = args.link {
    //     link(&args.filename, &output, &args.linkline)?;
    // }

    let mut lists: DataLists;
    let file_type: FileType;

    if let Some(x) = FileType::get_file_type_from_extension(
        &opt.input.extension().and_then(OsStr::to_str).unwrap(),
    ) {
        let file_type = x;
    }

    // players.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    // players.dedup_by(|a, b| a.name.eq_ignore_ascii_case(b.name));

    // println!("deserialize: {:#?}", players);
    // println!("deserialize: {:#?}", teams);

    //     if let Some(output) = opt.output {
    //         write_list_to_file(output, players);
    //     }
}