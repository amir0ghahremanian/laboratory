use std::{
    fs::OpenOptions,
    io::{ErrorKind, Read, Write},
};

use serde::{Deserialize, Serialize};

use crate::image::{Lab, StrResult};

#[derive(Serialize, Deserialize)]
pub struct Cache {
    labs: Vec<Lab>,
}

impl Cache {
    pub fn new(path: &str) -> Result<Self, String> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .str_result()?;

        let cache = Self {
            labs: Vec::new()
        };

        let toml = toml::to_string(&cache).unwrap();

        file.write_all(toml.as_bytes()).str_result()?;
        file.sync_all().str_result()?;

        Ok(cache)
    }

    pub fn load(path: &str) -> Result<Self, String> {
        let mut file = match OpenOptions::new().read(true).open(path) {
            Ok(t) => t,
            Err(e) => {
                match e.kind() {
                    ErrorKind::NotFound => { return Self::new(path); }
                    _ => { return Err(e.to_string()); }
                }
            }
        };

        let cache: Self = {
            let mut toml = String::new();

            file.read_to_string(&mut toml).str_result()?;

            toml::from_str(&toml).str_result()?
        };

        Ok(cache)
    }

    pub fn read(&mut self, path: &str) -> Result<(), String> {
        let mut file = OpenOptions::new().read(true).open(path).str_result()?;

        let cache: Self = {
            let mut toml = String::new();

            file.read_to_string(&mut toml).str_result()?;

            toml::from_str(&toml).str_result()?
        };

        *self = cache;

        Ok(())
    }

    pub fn sync(&self, path: &str) -> Result<(), String> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .str_result()?;

        let toml = toml::to_string(self).unwrap();

        file.write_all(toml.as_bytes()).str_result()?;
        file.sync_all().str_result()?;

        Ok(())
    }

    pub fn add(&mut self, lab: Lab) {
        self.labs.push(lab);
    }
}
