#![deny(clippy::all)]
#![deny(clippy::pedantic)]
// Relax compiler warnings
// TODO: Temporary, remove later
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(missing_docs)]
#![allow(dead_code)]
#![allow(unused_variables)]

use crate::datalists::{platforms, players, teams};

use serde::{de::DeserializeOwned, Serialize};

use std::{
    error::Error,
    ffi::OsStr,
    fs::File,
    io::{BufReader, BufWriter},
    path::{Path, PathBuf},
};

use crate::cli;
use log::{debug, error, info, trace, warn};
use stable_eyre::eyre::{eyre, Report, Result, WrapErr};

#[derive(Debug, Clone)]
pub struct DataLists {
    pub player_list: Vec<players::Players>,
    pub team_list: Vec<teams::Teams>,
    pub platform_list: Vec<platforms::Platforms>,
}

impl DataLists {
    pub fn new() -> DataLists {
        DataLists {
            player_list: Vec::new(),
            team_list: Vec::new(),
            platform_list: Vec::new(),
        }
    }

    // fn get_file_name_from_path(path: AsRef<Path>) -> Option<&str> {
    //     path.file_name().and_then(OsStr::to_str?)
    // }

    /// Fills members of a DataLists object with data deserialised from
    /// file paths that were given via the command line arguments
    /// players_input_path, teams_input_path, platforms_input_path
    pub fn new_from_cli_config(config: cli::Args) -> Result<DataLists, Report> {
        // TODO: Error handling
        let mut data = DataLists::new();

        // Deserializing Player file
        match config.players_input_path {
            Some(k) => {
                debug!("Player file given: {:?}", &k);
                data.player_list =
                    DataLists::deserialize_list_to_vec_from_file::<
                        players::Players,
                    >(&k)
                    .expect("Parsing of players file failed.");
            }
            None => {}
        };

        // Deserializing Teams file
        match config.teams_input_path {
            Some(k) => {
                debug!("Teams file given: {:?}", &k);
                data.team_list =
                DataLists::deserialize_list_to_vec_from_file::<teams::Teams>(
                    &k,
                )
                .expect("Parsing of teams file failed.");
            }
            None => {}
        };

        // Deserializing Platforms file
        match config.platforms_input_path {
            Some(k) => {
                debug!("Platforms file given: {:?}", &k);
                data.platform_list =
                    DataLists::deserialize_list_to_vec_from_file::<
                        platforms::Platforms,
                    >(&k)
                    .expect("Parsing of platforms file failed.");
            }
            None => {}
        }

        // Returning DataLists Struct
        Ok(data)
    }

    /// Generic function to deserialize a given file with `serde_any`
    /// from a path into a vector that contains a special datatype T  
    pub fn deserialize_list_to_vec_from_file<T>(
        path: &dyn AsRef<Path>
    ) -> Result<Vec<T>, Report>
    where
        T: DeserializeOwned,
    {
        // Open the file in read-only mode with buffer.
        // let file = File::open(&path)?;
        // let reader = BufReader::new(file);

        // Read the list contents of the file as an instance of `Vec<T>`.
        let list: Vec<T> = serde_any::from_file(path)
            .expect("Parsing of the data file failed.");
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

    /// Matches the extension of a given filename and
    /// returns the datatype for serde
    pub fn get_file_type_from_extension(
        ext: &str
    ) -> Option<serde_any::Format> {
        match ext {
            "yaml" | "yml" => Some(serde_any::Format::Yaml),
            "json" => Some(serde_any::Format::Json),
            "toml" => Some(serde_any::Format::Toml),
            _ => None,
        }
    }
}
