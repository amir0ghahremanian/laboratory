mod cache {
    use std::{
        fs::{create_dir_all, OpenOptions},
        io::{ErrorKind, Read, Write},
        path::Path,
        vec::IntoIter,
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

    impl IntoIterator for Cache {
        type Item = Lab;
        type IntoIter = IntoIter<Self::Item>;

        fn into_iter(self) -> Self::IntoIter {
            self.data.labs.into_iter()
        }
    }

    impl Cache {
        pub fn new(path: String) -> Result<Self, String> {
            let prefix = Path::new(&path).parent().unwrap();
            create_dir_all(prefix).str_result()?;

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

        // pub fn read(&mut self) -> Result<(), String> {
        //     let mut file = OpenOptions::new().read(true).open(&self.path).str_result()?;

        //     let cache_data: CacheData = {
        //         let mut toml = String::new();

        //         file.read_to_string(&mut toml).str_result()?;

        //         toml::from_str(&toml).str_result()?
        //     };

        //     self.data = cache_data;

        //     Ok(())
        // }

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

        pub fn add(&mut self, lab: Lab) -> Result<(), String> {
            for l in &self.data.labs {
                if l.config.name.eq(&lab.config.name) {
                    return Err("Lab with similar name exists!".to_string());
                }
            }

            self.data.labs.push(lab);

            Ok(())
        }

        pub fn search(&mut self, name: &str) -> Result<&mut Lab, String> {
            for l in &mut self.data.labs {
                if l.config.name.eq(name) {
                    return Ok(l);
                }
            }

            Err("Lab not found!".to_string())
        }

        pub fn remove(&mut self, name: &str) -> Result<(), String> {
            let index = self.data.labs.iter().position(|x| x.config.name.eq(name));

            match index {
                Some(index) => {
                    self.data.labs.remove(index);
                    Ok(())
                }
                None => Err("Lab not found!".to_string()),
            }
        }
    }
}

pub mod manage {
    use std::{
        env::{self, current_dir},
        path::Path,
    };

    use crate::image::{Lab, StrResult};

    use super::cache::Cache;

    const CACHE_PATH_APPDATA: &str = "laboratory\\Cache.toml";

    pub fn import_lab(image: String, config: String) -> Result<(), String> {
        let mut lab = Lab::from_image(analyze_path(image)?);

        lab.read_config(&config)?;

        let mut cache = Cache::load(cache_path())?;

        cache.add(lab)?;
        cache.write()?;

        Ok(())
    }

    pub fn list() -> Result<(), String> {
        let cache = Cache::load(cache_path())?;

        for lab in cache {
            println!("{}", lab.config.name);
        }

        Ok(())
    }

    pub fn list_apps(name: String) -> Result<(), String> {
        let mut cache = Cache::load(cache_path())?;

        let lab = cache.search(&name)?;

        for app in &lab.config.apps {
            println!("{}", app.name);
        }

        Ok(())
    }

    pub fn run(name: String, app: String, drive_letter: Option<String>) -> Result<(), String> {
        let mut cache = Cache::load(cache_path())?;

        let lab = cache.search(&name)?;

        match drive_letter {
            Some(drive_letter) => {
                lab.mount(drive_letter)?;
                cache.write()?;
            }
            None => {}
        };

        // repetition is not ideal
        let lab = cache.search(&name)?;

        let mut child = lab.run(&app)?;
        child.wait().str_result()?;

        Ok(())
    }

    pub fn expand(name: String, path: String) -> Result<(), String> {
        let mut cache = Cache::load(cache_path())?;

        let lab = cache.search(&name)?;

        if let Some(_) = &lab.drive_letter {
            return Err("Lab is mounted!".to_string());
        }

        lab.expand(analyze_path(path)?)?;

        cache.write()?;

        Ok(())
    }

    pub fn repack(name: String) -> Result<(), String> {
        let mut cache = Cache::load(cache_path())?;

        let lab = cache.search(&name)?;

        if let Some(_) = &lab.drive_letter {
            return Err("Lab is mounted!".to_string());
        }

        lab.repack()?;

        cache.write()?;

        Ok(())
    }

    pub fn restore(name: String) -> Result<(), String> {
        let mut cache = Cache::load(cache_path())?;

        let lab = cache.search(&name)?;

        if let Some(_) = &lab.drive_letter {
            return Err("Lab is mounted!".to_string());
        }

        lab.restore()?;

        Ok(())
    }

    pub fn remove(name: String) -> Result<(), String> {
        let mut cache = Cache::load(cache_path())?;

        cache.remove(&name)?;
        cache.write()?;

        Ok(())
    }

    pub fn change(name: String, image: String) -> Result<(), String> {
        let mut cache = Cache::load(cache_path())?;

        let lab = cache.search(&name)?;
        lab.image_path = Some(analyze_path(image)?);

        cache.write()?;

        Ok(())
    }

    pub fn update(name: String, path: String) -> Result<(), String> {
        let mut cache = Cache::load(cache_path())?;

        let lab = cache.search(&name)?;
        lab.read_config(&path)?;

        cache.write()?;

        Ok(())
    }

    pub fn mount(name: String, drive_letter: String) -> Result<(), String> {
        let mut cache = Cache::load(cache_path())?;

        let lab = cache.search(&name)?;
        lab.mount(drive_letter)?;

        cache.write()?;

        Ok(())
    }

    pub fn unmount(name: String) -> Result<(), String> {
        let mut cache = Cache::load(cache_path())?;

        let lab = cache.search(&name)?;
        lab.unmount()?;

        cache.write()?;

        Ok(())
    }

    fn analyze_path(path_str: String) -> Result<String, String> {
        let path = Path::new(&path_str);

        if !path.has_root() {
            let mut path = current_dir()
                .str_result()?
                .into_os_string()
                .into_string()
                .unwrap();
            path = path + "\\" + &path_str;

            return Ok(path);
        }

        Ok(path_str)
    }

    #[inline(always)]
    fn cache_path() -> String {
        env::var("APPDATA").unwrap() + "\\" + CACHE_PATH_APPDATA
    }
}
