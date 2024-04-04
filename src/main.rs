mod cmd;
mod image;
mod manager;

use std::env::args;

use cmd::{parse_args, usage_and_exit, RunOptions::*};
use manager::manage;

fn main() -> Result<(), String> {
    let mut args = args();
    args.next();

    let run_options = parse_args(args)?;

    match run_options {
        Exit => {}
        New(lab_name) => {
            println!("name = {}", lab_name);
        }
        Import(config, image) => {
            manage::import_lab(
                match image {
                    Some(image) => image,
                    None => { usage_and_exit!(); }
                },
                config
            )?;
        }
        List => {
            manage::list()?;
        }
        Run(name, app, drive_letter) => {
            manage::run(
                name,
                match app {
                    Some(app) => app,
                    None => { usage_and_exit!(); }
                },
                match drive_letter {
                    Some(drive_letter) => drive_letter,
                    None => { usage_and_exit!(); }
                }
            )?;
        }
    };

    Ok(())
}
