mod cmd;
mod image;
mod manager;

use std::env::args;

use cmd::{parse_args, print_usage, RunOptions::*};
use manager::manage;

fn main() -> Result<(), String> {
    let mut args = args();
    args.next();

    let run_options = parse_args(args)?;

    match run_options {
        Exit => {
            return Ok(());
        }
        New(lab_name) => {
            println!("name = {}", lab_name);
        }
        Import(config, image) => {
            if let Some(image) = image {
                manage::import_lab(image, config)?;
            } else {
                print_usage();
            }
        }
        List => {

        }
    };

    Ok(())
}
