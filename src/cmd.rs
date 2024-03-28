use std::env::Args;

pub enum RunOptions {
    Exit,
    New(String),
    Import(String, Option<String>),
    List,
}

#[inline(always)]
pub fn parse_args(mut args: Args) -> Result<RunOptions, String> {
    if args.len() == 0 {
        print_usage();

        return Ok(RunOptions::Exit);
    }

    let mut output = RunOptions::Exit;

    while let Some(arg) = args.next() {
        if arg.eq("-v") || arg.eq("--version") {
            print_version();

            return Ok(RunOptions::Exit);
        } else if arg.eq("-n") || arg.eq("--new") {
            output = RunOptions::New(match args.next() {
                Some(t) => t,
                None => {
                    print_usage();

                    return Ok(RunOptions::Exit);
                }
            });

            continue;
        } else if arg.eq("-I") || arg.eq("--import") {
            output = RunOptions::Import(
                match args.next() {
                    Some(t) => t,
                    None => {
                        print_usage();

                        return Ok(RunOptions::Exit);
                    }
                },
                None,
            );

            continue;
        } else if arg.eq("-i") || arg.eq("--image") {
            if let RunOptions::Import(_, image) = &mut output {
                *image = match args.next() {
                    Some(t) => Some(t),
                    None => {
                        print_usage();

                        return Ok(RunOptions::Exit);
                    }
                };
            } else {
                print_usage();

                return Ok(RunOptions::Exit);
            }

            continue;
        } else if arg.eq("-l") || arg.eq("--list") {
            return Ok(RunOptions::List);
        } else {
            print_usage();

            return Ok(RunOptions::Exit);
        }
    }

    Ok(output)
}

#[inline(always)]
pub fn print_version() {
    println!("Laboratory v0.1.0");
}

#[inline(always)]
pub fn print_usage() {
    println!("Usage:");
}
