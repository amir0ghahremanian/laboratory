use std::env::Args;

pub enum RunOptions {
    Exit,
    Import(String, Option<String>),
    List,
    ListApps(String),
    Run(String, Option<String>, Option<String>),
    Change(String, Option<String>),
    Expand(String, Option<String>),
    Repack(String),
    Remove(String),
    Mount(String, Option<String>),
    Unmount(String)
}

#[inline(always)]
pub fn parse_args(mut args: Args) -> Result<RunOptions, String> {
    if args.len() == 0 { usage_and_return!(); }

    let mut output = RunOptions::Exit;

    while let Some(arg) = args.next() {
        if arg.eq("-v") || arg.eq("--version") {
            print_version();

            return Ok(RunOptions::Exit);
        } else if arg.eq("-I") || arg.eq("--import") {
            output = RunOptions::Import(
                match args.next() {
                    Some(t) => t,
                    None => { usage_and_return!(); }
                },
                None,
            );

            continue;
        } else if arg.eq("-i") || arg.eq("--image") {
            if let RunOptions::Import(_, image) = &mut output {
                *image = match args.next() {
                    Some(t) => Some(t),
                    None => { usage_and_return!(); }
                };
            } else if let RunOptions::Change(_, image) = &mut output {
                *image = match args.next() {
                    Some(t) => Some(t),
                    None => { usage_and_return!(); }
                };
            } else { usage_and_return!(); }

            continue;
        } else if arg.eq("-R") || arg.eq("--run") {
            output = RunOptions::Run(
                match args.next() {
                    Some(t) => t,
                    None => { usage_and_return!(); }
                },
                None,
                None
            );

            continue;
        } else if arg.eq("-a") || arg.eq("--app") {
            if let RunOptions::Run(_, app, _) = &mut output {
                *app = match args.next() {
                    Some(t) => Some(t),
                    None => { usage_and_return!(); }
                };
            } else { usage_and_return!(); }

            continue;
        } else if arg.eq("-d") || arg.eq("--drive-letter") {
            if let RunOptions::Run(_, _, drive_letter) = &mut output {
                *drive_letter = match args.next() {
                    Some(t) => Some(t),
                    None => { usage_and_return!(); }
                };
            } else if let RunOptions::Mount(_, drive_letter) = &mut output {
                *drive_letter = match args.next() {
                    Some(t) => Some(t),
                    None => { usage_and_return!(); }
                };
            } else { usage_and_return!(); }

            continue;
        } else if arg.eq("-c") || arg.eq("--change") {
            output = RunOptions::Change(
                match args.next() {
                    Some(t) => t,
                    None => { usage_and_return!(); }
                },
                None
            );

            continue;
        } else if arg.eq("-e") || arg.eq("--expand") {
            output = RunOptions::Expand(
                match args.next() {
                    Some(t) => t,
                    None => { usage_and_return!(); }
                },
                None
            );

            continue;
        } else if arg.eq("-p") || arg.eq("--path") {
            if let RunOptions::Expand(_, path) = &mut output {
                *path = match args.next() {
                    Some(t) => Some(t),
                    None => { usage_and_return!(); }
                };
            } else { usage_and_return!(); }

            continue;
        } else if arg.eq("-m") || arg.eq("--mount") {
            output = RunOptions::Mount(
                match args.next() {
                    Some(t) => t,
                    None => { usage_and_return!(); }
                },
                None
            );

            continue;
        } else if arg.eq("-u") || arg.eq("--unmount") {
            output = RunOptions::Unmount(match args.next() {
                Some(t) => t,
                None => { usage_and_return!(); }
            });

            continue;
        } else if arg.eq("-r") || arg.eq("--repack") {
            output = RunOptions::Repack(match args.next() {
                Some(t) => t,
                None => { usage_and_return!(); }
            });

            continue;
        } else if arg.eq("-rm") || arg.eq("--remove") {
            output = RunOptions::Remove(match args.next() {
                Some(t) => t,
                None => { usage_and_return!(); }
            });

            continue;
        } else if arg.eq("-L") || arg.eq("--list-apps") {
            output = RunOptions::ListApps(match args.next() {
                Some(t) => t,
                None => { usage_and_return!(); }
            });

            continue;
        } else if arg.eq("-l") || arg.eq("--list") {
            return Ok(RunOptions::List);
        } else { usage_and_return!(); }
    }

    Ok(output)
}

#[inline(always)]
pub fn print_version() {
    println!("Laboratory v0.1.1");
}

#[inline(always)]
pub fn print_usage() {
    println!("Usage:");
}

macro_rules! usage_and_return {
    () => {
        crate::cmd::print_usage();

        return Ok(crate::cmd::RunOptions::Exit);
    };
}

macro_rules! usage_and_exit {
    () => {
        crate::cmd::print_usage();

        return Ok(());
    };
}

pub(crate) use {usage_and_exit, usage_and_return};
