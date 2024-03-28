mod cache {
    use std::{
        fs::OpenOptions,
        io::{ErrorKind, Read, Write},
    };

    use serde::{Deserialize, Serialize};

    use crate::image::{Lab, StrResult};

    pub struct Cache {
        data: CacheData,
        path: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct CacheData {
        labs: Vec<Lab>,
    }

    impl Cache {
        // cannot handle missing folder yet
        pub fn new(path: String) -> Result<Self, String> {
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&path)
                .str_result()?;

            let cache = Self {
                data: CacheData { labs: Vec::new() },
                path,
            };

            let toml = toml::to_string(&cache.data).unwrap();

            file.write_all(toml.as_bytes()).str_result()?;
            file.sync_all().str_result()?;

            Ok(cache)
        }

        pub fn load(path: String) -> Result<Self, String> {
            let mut file = match OpenOptions::new().read(true).open(&path) {
                Ok(t) => t,
                Err(e) => match e.kind() {
                    ErrorKind::NotFound => {
                        return Self::new(path);
                    }
                    _ => {
                        return Err(e.to_string());
                    }
                },
            };

            let cache_data: CacheData = {
                let mut toml = String::new();

                file.read_to_string(&mut toml).str_result()?;

                toml::from_str(&toml).str_result()?
            };

            Ok(Self {
                data: cache_data,
                path,
            })
        }

        pub fn read(&mut self) -> Result<(), String> {
            let mut file = OpenOptions::new().read(true).open(&self.path).str_result()?;

            let cache_data: CacheData = {
                let mut toml = String::new();

                file.read_to_string(&mut toml).str_result()?;

                toml::from_str(&toml).str_result()?
            };

            self.data = cache_data;

            Ok(())
        }

        pub fn write(&self) -> Result<(), String> {
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&self.path)
                .str_result()?;

            let toml = toml::to_string(&self.data).unwrap();

            file.write_all(toml.as_bytes()).str_result()?;
            file.sync_all().str_result()?;

            Ok(())
        }

        pub fn add(&mut self, lab: Lab) {
            self.data.labs.push(lab);
        }
    }
}

pub mod manage {
    use std::env;

    use crate::image::Lab;

    use super::cache::Cache;

    const CACHE_PATH_APPDATA: &str = "laboratory\\Cache.toml";

    pub fn import_lab(image: String, config: String) -> Result<(), String> {
        let mut lab = Lab::from_image(image);

        lab.read_config(&config)?;

        let mut cache = Cache::load(cache_path())?;

        cache.add(lab);
        cache.write()
    }

    #[inline(always)]
    fn cache_path() -> String {
        env::var("APPDATA").unwrap() + "\\" + CACHE_PATH_APPDATA
    }
}
