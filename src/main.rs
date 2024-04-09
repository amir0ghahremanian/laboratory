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
        ListApps(name) => {
            manage::list_apps(name)?;
        }
        Run(name, app, drive_letter) => {
            manage::run(
                name,
                match app {
                    Some(app) => app,
                    None => { usage_and_exit!(); }
                },
                drive_letter
            )?;
        }
        Change(name, image) => {
            manage::change(
                name,
                match image {
                    Some(image) => image,
                    None => { usage_and_exit!(); }
                }
            )?;
        }
        Update(name, path) => {
            manage::update(
                name,
                match path {
                    Some(path) => path,
                    None => { usage_and_exit!(); }
                }
            )?;
        }
        Expand(name, path) => {
            manage::expand(
                name,
                match path {
                    Some(path) => path,
                    None => { usage_and_exit!(); }
                }
            )?;
        }
        Discard(name) => {
            manage::discard(name)?;
        }
        Repack(name) => {
            manage::repack(name)?;
        }
        Restore(name) => {
            manage::restore(name)?;
        }
        Remove(name) => {
            manage::remove(name)?;
        }
        Mount(name, drive_letter) => {
            manage::mount(
                name,
                match drive_letter {
                    Some(drive_letter) => drive_letter,
                    None => { usage_and_exit!(); }
                }
            )?;
        }
        Unmount(name) => {
            manage::unmount(name)?;
        }
    };

    Ok(())
}
