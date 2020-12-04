#![deny(clippy::all)]
#![deny(clippy::pedantic)]

mod cli;
mod players;
mod teams;

use crate::cli::*;
use crate::players::*;
use crate::teams::*;

use ::serde::{de::DeserializeOwned, Deserialize, Serialize};

use std::error::Error;
use std::ffi::OsStr;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::path::Path;

use structopt::StructOpt;

#[derive(Debug)]
pub enum FileType {
    Json,
    Toml,
    Yaml,
}

#[derive(Debug, Deserialize)]
pub enum ContentListType {
    Players,
    Teams,
}

fn main() {
    let opt = Args::from_args();
    println!("{:#?}", opt);

    // if let Some(output) = args.link {
    //     link(&args.filename, &output, &args.linkline)?;
    // }

    let file_type = opt
        .input
        .extension()
        .and_then(OsStr::to_str)
        .and_then(|ext| match ext {
            "yaml" | "yml" => Some(FileType::Yaml),
            "json" => Some(FileType::Json),
            "toml" => Some(FileType::Toml),
            _ => None,
        });

    let data: Option<Vec<ContentListType>> = opt
        .input
        .file_name()
        .and_then(OsStr::to_str)
        .and_then(|filename| match filename {
            "players" => {
                Some(read_list_from_file(opt.input, file_type, ContentListType::Players).unwrap())
            }
            "teams" => {
                Some(read_list_from_file(opt.input, file_type, ContentListType::Teams).unwrap())
            }
            _ => None,
        });

    // players.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    // players.dedup_by(|a, b| a.name.eq_ignore_ascii_case(b.name));

    // println!("deserialize: {:#?}", players);
    // println!("deserialize: {:#?}", teams);

    //     if let Some(output) = opt.output {
    //         write_list_to_file(output, players);
    //     }
}

fn read_list_from_file<P, T>(
    path: P,
    ftype: Option<FileType>,
    ctype: ContentListType,
) -> Result<Vec<T>, Box<dyn Error>>
where
    P: AsRef<Path>,
    T: DeserializeOwned,
{
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    match ftype {
        Some(FileType::Yaml) => {
            match ctype {
                ContentListType::Players => {
                    // Read the YAML contents of the file as an instance of `ContentListType`.
                    let list: Vec<Players> = serde_yaml::from_reader(reader)?;

                    // Return the parsed data list
                    Ok(list)
                }
                ContentListType::Teams => {
                    // Read the YAML contents of the file as an instance of `ContentListType`.
                    let mut list: Vec<Teams> = serde_yaml::from_reader(reader)?;

                    // Return the parsed data list
                    Ok(list)
                }
            }
        }
        Some(FileType::Json) => {
            match ctype {
                ContentListType::Players => {
                    // Read the YAML contents of the file as an instance of `ContentListType`.
                    let mut list: Vec<Players> = serde_json::from_reader(reader)?;

                    // Return the parsed data list
                    Ok(list)
                }
                ContentListType::Teams => {
                    // Read the YAML contents of the file as an instance of `ContentListType`.
                    let mut list: Vec<Teams> = serde_json::from_reader(reader)?;

                    // Return the parsed data list
                    Ok(list)
                }
            }
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
