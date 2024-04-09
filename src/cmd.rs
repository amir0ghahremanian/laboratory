use std::env::Args;

use colored::*;

pub enum RunOptions {
    Exit,
    Import(String, Option<String>),
    List,
    ListApps(String),
    Run(String, Option<String>, Option<String>),
    Change(String, Option<String>),
    Update(String, Option<String>),
    Expand(String, Option<String>),
    Discard(String),
    Repack(String),
    Restore(String),
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
            } else if let RunOptions::Update(_, path) = &mut output {
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
        } else if arg.eq("-rs") || arg.eq("--restore") {
            output = RunOptions::Restore(match args.next() {
                Some(t) => t,
                None => { usage_and_return!(); }
            });

            continue;
        } else if arg.eq("-U") || arg.eq("--update") {
            output = RunOptions::Update(
                match args.next() {
                    Some(t) => t,
                    None => { usage_and_return!(); }
                },
                None
            );

            continue;
        } else if arg.eq("-D") || arg.eq("--discard") {
            output = RunOptions::Discard(match args.next() {
                Some(t) => t,
                None => { usage_and_return!(); }
            });

            continue;
        } else if arg.eq("-l") || arg.eq("--list") {
            return Ok(RunOptions::List);
        } else {
            println!("Unknown option: {}", arg.red().bold());

            usage_and_return!();
        }
    }

    Ok(output)
}

#[inline(always)]
pub fn print_version() {
    println!(
r#"Laboratory v0.1.4
Copyright (C) 2023 amir0ghahremanian
This is free software; see the source for copying conditions.  There is NO
warranty; not even for MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
"#
    );
}

pub fn print_usage() {
    println!("\n{} {} {}\n", "Usage:".green().bold(), "laboratory".cyan().bold(), "[OPTIONS]".cyan());

    println!("{}", "Options:".green().bold());
    print!("  {}, {}", "-v".cyan().bold(), "--version".cyan().bold());
    println!("                     Print version info and exit");
    print!("  {}, {} {} {}", "-I".cyan().bold(), "--import".cyan().bold(), "<CONFIG>".cyan(), "[IMAGE]".cyan());
    println!("     Import laboratory");
    print!("  {}, {} {}", "-i".cyan().bold(), "--image".cyan().bold(), "<IMAGE>".cyan());
    println!("               Choose image");
    print!("  {}, {} {} {}", "-R".cyan().bold(), "--run".cyan().bold(), "<LAB>".cyan(), "[APP]".cyan());
    println!("             Run app from laboratory");
    print!("  {}, {} {}", "-a".cyan().bold(), "--app".cyan().bold(), "<APP>".cyan());
    println!("                   Choose app");
    print!("  {}, {} {}", "-d".cyan().bold(), "--drive-letter".cyan().bold(), "<LETTER>".cyan());
    println!("       Choose drive letter");
    print!("  {}, {} {} {}", "-c".cyan().bold(), "--change".cyan().bold(), "<LAB>".cyan(), "[IMAGE]".cyan());
    println!("        Change laboratory image");
    print!("  {}, {} {} {}", "-U".cyan().bold(), "--update".cyan().bold(), "<LAB>".cyan(), "[PATH]".cyan());
    println!("         Update laboratory configuration");
    print!("  {}, {} {} {}", "-e".cyan().bold(), "--expand".cyan().bold(), "<LAB>".cyan(), "[PATH]".cyan());
    println!("         Expand laboratory");
    print!("  {}, {} {}", "-p".cyan().bold(), "--path".cyan().bold(), "<PATH>".cyan());
    println!("                 Choose path");
    print!("  {}, {} {} {}", "-m".cyan().bold(), "--mount".cyan().bold(), "<LAB>".cyan(), "[LETTER]".cyan());
    println!("        Mount laboratory");
    print!("  {}, {} {}", "-u".cyan().bold(), "--unmount".cyan().bold(), "<LAB>".cyan());
    println!("               Unmount laboratory");
    print!("  {}, {} {}", "-D".cyan().bold(), "--discard".cyan().bold(), "<LAB>".cyan());
    println!("               Discard and remove expanded folder");
    print!("  {}, {} {}", "-r".cyan().bold(), "--repack".cyan().bold(), "<LAB>".cyan());
    println!("                Repack laboratory");
    print!("  {}, {} {}", "-rs".cyan().bold(), "--restore".cyan().bold(), "<LAB>".cyan());
    println!("              Restore laboratory");
    print!("  {}, {} {}", "-rm".cyan().bold(), "--remove".cyan().bold(), "<LAB>".cyan());
    println!("               Remove laboratory");
    print!("  {}, {}", "-l".cyan().bold(), "--list".cyan().bold());
    println!("                        List laboratories");
    print!("  {}, {} {}", "-L".cyan().bold(), "--list-apps".cyan().bold(), "<LAB>".cyan());
    println!("             List apps");

    println!("");
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
