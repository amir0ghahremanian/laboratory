use std::{
    fs::OpenOptions,
    io::{self, Read, Write},
};

use serde::{Deserialize, Serialize};
use toml::de::Error;

#[derive(Serialize, Deserialize)]
pub struct Lab {
    image_path: Option<String>,
    expanded_path: Option<String>,
    drive_letter: Option<String>,
    config: Option<LabConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct LabConfig {
    name: String,
    apps: Vec<App>,
}

#[derive(Serialize, Deserialize)]
pub struct App {
    name: String,
}

impl Lab {
    #[inline(always)]
    pub fn from_image(path: String) -> Self {
        Self {
            image_path: Some(path),
            expanded_path: None,
            drive_letter: None,
            config: None,
        }
    }

    #[inline(always)]
    pub fn from_expanded(path: String) -> Self {
        Self {
            image_path: None,
            expanded_path: Some(path),
            drive_letter: None,
            config: None
        }
    }

    pub fn read_config(&mut self, path: &str) -> Result<(), String> {
        let mut file = OpenOptions::new().read(true).open(path).str_result()?;

        let config: LabConfig = {
            let mut toml = String::new();

            file.read_to_string(&mut toml).str_result()?;

            toml::from_str(&toml).str_result()?
        };

        self.config = Some(config);

        Ok(())
    }

    pub fn write_config(&self, path: &str) -> Result<(), String> {
        if let Some(config) = &self.config {
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(path)
                .str_result()?;

            let toml = toml::to_string(config).unwrap();

            file.write_all(toml.as_bytes()).str_result()?;
            file.sync_all().str_result()?;

            return Ok(());
        }

        Err("No config to write!".to_string())
    }

    pub fn expand(&mut self, target_path: String) -> Result<(), String> {
        Err("Not implemented!".to_string())
    }

    pub fn mount(&mut self, drive_letter: String) -> Result<(), String> {
        if let Some(expanded_path) = &self.expanded_path {
            match Self::create_volume(&drive_letter, &expanded_path) {
                true => {
                    self.drive_letter = Some(drive_letter);

                    return Ok(());
                }
                false => return Err("Failed to create volume!".to_string()),
            }
        }

        Err("Lab not expanded!".to_string())
    }

    pub fn unmount(&mut self) -> Result<(), String> {
        if let Some(drive_letter) = &self.drive_letter {
            match Self::delete_volume(&drive_letter) {
                true => {
                    self.drive_letter = None;

                    return Ok(());
                }
                false => return Err("Failed to delete volume!".to_string()),
            }
        }

        Err("Lab not mounted!".to_string())
    }

    #[inline(always)]
    fn create_volume(drive_letter: &str, path: &str) -> bool {
        win_subst::add(drive_letter, path)
    }

    #[inline(always)]
    fn delete_volume(drive_letter: &str) -> bool {
        win_subst::del(drive_letter)
    }
}

pub trait StrResult<T> {
    fn str_result(self) -> Result<T, String>;
}

impl<T> StrResult<T> for Result<T, Error> {
    fn str_result(self) -> Result<T, String> {
        match self {
            Ok(t) => Ok(t),
            Err(e) => Err(e.to_string()),
        }
    }
}

impl<T> StrResult<T> for io::Result<T> {
    fn str_result(self) -> Result<T, String> {
        match self {
            Ok(t) => Ok(t),
            Err(e) => Err(e.to_string()),
        }
    }
}
