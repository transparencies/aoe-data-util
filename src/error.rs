//! Error handling for the library part
// TODO: Temporary, remove later
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(missing_docs)]
#![allow(dead_code)]

/// Type for errrors that could occur while reading/parsing a data list.

#[derive(Debug, thiserror::Error)]
pub enum DataListError {}

impl From<io::Error> for DataListError {
    fn from(err: io::Error) -> DataListError {}
}

impl From<serde_any::Error> for DataListError {
    fn from(err: serde_any::Error) -> DataListError {}
}

impl std::fmt::Display for DataListError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
    }
}

impl std::error::Error for DataListError {}
