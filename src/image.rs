use std::{
    collections::HashMap, env, fs::{remove_dir_all, OpenOptions}, io::{self, Read}, process::{Child, Command}
};

use serde::{Deserialize, Serialize};
use tar::{Archive, Builder};
use toml::de::Error;

#[derive(Serialize, Deserialize)]
pub struct Lab {
    pub image_path: Option<String>,
    pub expanded_path: Option<String>,
    pub drive_letter: Option<String>,
    pub config: Option<LabConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct LabConfig {
    pub name: String,
    pub apps: Vec<App>,
}

#[derive(Serialize, Deserialize)]
pub struct App {
    pub name: String,
    command: String,
    work_dir: String,
    envs: Vec<Env>,
}

#[derive(Serialize, Deserialize)]
pub struct Env {
    key: String,
    value: String
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

    // #[inline(always)]
    // pub fn from_expanded(path: String) -> Self {
    //     Self {
    //         image_path: None,
    //         expanded_path: Some(path),
    //         drive_letter: None,
    //         config: None,
    //     }
    // }

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

    // pub fn write_config(&self, path: &str) -> Result<(), String> {
    //     if let Some(config) = &self.config {
    //         let mut file = OpenOptions::new()
    //             .write(true)
    //             .create(true)
    //             .truncate(true)
    //             .open(path)
    //             .str_result()?;

    //         let toml = toml::to_string(config).unwrap();

    //         file.write_all(toml.as_bytes()).str_result()?;
    //         file.sync_all().str_result()?;

    //         return Ok(());
    //     }

    //     Err("No config to write!".to_string())
    // }

    pub fn repack(&mut self) -> Result<(), String> {
        if let Some(expanded_path) = &self.expanded_path {
            if let Some(image_path) = &self.image_path {
                let mut archive = Builder::new(
                    OpenOptions::new()
                        .write(true)
                        .create(true)
                        .truncate(true)
                        .open(image_path)
                        .str_result()?,
                );

                archive.append_dir_all(".", expanded_path).str_result()?;
                archive.into_inner().str_result()?.sync_all().str_result()?;

                remove_dir_all(expanded_path).str_result()?;

                self.expanded_path = None;

                return Ok(());
            }

            return Err("No image to repack!".to_string());
        }

        Err("Lab not expanded!".to_string())
    }

    pub fn restore(&self) -> Result<(), String> {
        if let Some(expanded_path) = &self.expanded_path {
            if let Some(image_path) = &self.image_path {
                remove_dir_all(expanded_path).str_result()?;

                let mut archive = Archive::new(
                    OpenOptions::new()
                        .read(true)
                        .open(image_path)
                        .str_result()?,
                );

                archive.unpack(expanded_path).str_result()?;

                return Ok(());
            }

            return Err("No image to restore!".to_string());
        }

        Err("Lab not expanded!".to_string())
    }

    pub fn expand(&mut self, target_path: String) -> Result<(), String> {
        if let Some(image_path) = &self.image_path {
            let mut archive = Archive::new(
                OpenOptions::new()
                    .read(true)
                    .open(image_path)
                    .str_result()?,
            );

            archive.unpack(&target_path).str_result()?;

            self.expanded_path = Some(target_path);

            return Ok(());
        }

        Err("No image to expand!".to_string())
    }

    pub fn mount(&mut self, drive_letter: String) -> Result<(), String> {
        match &self.drive_letter {
            Some(d) => {
                if !d.eq(&drive_letter) {
                    self.unmount()?;
                } else {
                    return Ok(());
                }
            }
            None => {}
        };

        if let Some(expanded_path) = &self.expanded_path {
            match Self::create_volume((drive_letter.clone() + ":").as_str(), &expanded_path) {
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
            match Self::delete_volume((drive_letter.clone() + ":").as_str()) {
                true => {
                    self.drive_letter = None;

                    return Ok(());
                }
                false => return Err("Failed to delete volume!".to_string()),
            }
        }

        Err("Lab not mounted!".to_string())
    }

    pub fn run(&self, app: &str) -> Result<Child, String> {
        if let Some(drive_letter) = &self.drive_letter {
            if let Some(c) = &self.config {
                for a in &c.apps {
                    if a.name.eq(app) {
                        // run app and return handle
                        let child = Command::new(drive_letter.clone() + ":" + &a.command)
                            .env_clear()
                            .current_dir(drive_letter.clone() + ":" + &a.work_dir)
                            .envs(self.analyze_envs(&a))
                            .spawn()
                            .str_result()?;

                        return Ok(child);
                    }
                }

                return Err("App not found!".to_string());
            }
        }

        Err("Lab not mounted".to_string())
    }

    fn analyze_envs(&self, app: &App) -> HashMap<String, String> {
        let mut analyzed: HashMap<String, String> = HashMap::new();

        let drive_letter = self.drive_letter.as_ref().unwrap();

        for env in &app.envs {
            let (key, mut value) = (env.key.clone(), env.value.clone());

            // key = key.replace("$mnt$", &drive_letter);
            if value.eq("$sm$") {
                value = env::var(&key).unwrap();
            } else {
                value = value.replace("$mnt$", &drive_letter);
            }

            analyzed.insert(key, value);
        }

        analyzed
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
