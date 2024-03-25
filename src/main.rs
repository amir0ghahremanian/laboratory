mod cmd;
mod image;
mod manager;

use std::env::{self, args};

use cmd::parse_args;
use image::Lab;
use manager::Cache;

const CACHE_PATH_APPDATA: &str = "laboratory\\Cache.toml";

fn main() -> Result<(), String> {
    let mut args = args();
    args.next();

    let run_options = parse_args(args)?;

    if run_options.exit {
        return Ok(());
    }

    if let Some(lab_name) = run_options.new {
        println!("name = {}", lab_name);
    } else if let Some(config) = run_options.import {
        if let Some(image) = run_options.image {
            let mut lab = Lab::from_image(image);

            lab.read_config(&config)?;

            let cache_path = cache_path();

            let mut cache = Cache::load(&cache_path)?;

            cache.add(lab);
            cache.sync(&cache_path)?;
        }
    }

    Ok(())
}

fn cache_path() -> String {
    env::var("APPDATA").unwrap() + "\\" + CACHE_PATH_APPDATA
}
