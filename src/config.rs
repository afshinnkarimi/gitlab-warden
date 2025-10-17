use serde::Deserialize;
use serde_yaml;
use std::{fs::File, io, path::Path};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: String,
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = File::open(&path)?;
        serde_yaml::from_reader(file)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
}