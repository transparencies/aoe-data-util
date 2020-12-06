#![deny(clippy::all)]
#![deny(clippy::pedantic)]
// TODO: Temporary, remove later
#![allow(dead_code)]
#![allow(unused_imports)]

use crate::datalists::{platforms, players, teams};

use serde::{de::DeserializeOwned, Serialize};

use std::ffi::OsStr;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::path::Path;
use std::{error::Error, path::PathBuf};

use log::{debug, error, info, trace, warn};
use stable_eyre::eyre::{eyre, Report, Result, WrapErr};

#[derive(Debug, Clone)]
pub struct DataLists {
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

    pub fn read_list_from_file<T>(
        path: &dyn AsRef<Path>,
        //    ftype: Option<FileType>,
    ) -> Result<Vec<T>, Box<dyn Error>>
    where
        T: DeserializeOwned,
    {
        // Open the file in read-only mode with buffer.
        // let file = File::open(&path)?;
        // let reader = BufReader::new(file);

        // Read the list contents of the file as an instance of `Vec<T>`.
        let list: Vec<T> = serde_any::from_file(path).expect("Parsing of the data file failed.");
        Ok(list)
    }

    pub fn write_list_to_file<P, S>(
        path: P,
        obj: S,
        fmt: serde_any::Format,
    ) -> Result<(), std::io::Error>
    where
        P: AsRef<Path>,
        S: Serialize,
    {
        // Open the file in writable mode with buffer.
        let file = File::create(path)?;
        let writer = BufWriter::new(file);

        // Write data to file
        serde_any::to_writer(writer, &obj, fmt).expect("Unable to write data");

        Ok(())
    }

    pub fn get_file_type_from_extension(ext: &str) -> Option<serde_any::Format> {
        match ext {
            "yaml" | "yml" => Some(serde_any::Format::Yaml),
            "json" => Some(serde_any::Format::Json),
            "toml" => Some(serde_any::Format::Toml),
            _ => None,
        }
    }
}
