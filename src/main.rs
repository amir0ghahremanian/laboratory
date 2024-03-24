mod image;

use std::env::{args, Args};

use image::Lab;

struct RunOptions {
    exit: bool
}

fn main() -> Result<(), String> {
    let mut args = args();
    args.next();

    let run_options = parse_args(args)?;

    if run_options.exit {
        return Ok(());
    }

    Lab::from_image("test".to_string()).expand("test".to_string())
}

#[inline(always)]
fn parse_args(args: Args) -> Result<RunOptions, String> {
    let mut run_options = RunOptions {
        exit: false
    };

    if args.len() == 0 {
        print_usage();

        run_options.exit = true;
        return Ok(run_options);
    }

    for arg in args {
        if arg.eq("-v") || arg.eq("--version") {
            print_version();

            run_options.exit = true;
            return Ok(run_options);
        } else if arg.eq("-t") || arg.eq("--test") {
            continue;
        } else {
            print_usage();

            run_options.exit = true;
            return Ok(run_options);
        }
    }

    Ok(run_options)
}

#[inline(always)]
fn print_version() {
    println!("Laboratory v0.1.0");
}

#[inline(always)]
fn print_usage() {
    println!("Usage:");
}
