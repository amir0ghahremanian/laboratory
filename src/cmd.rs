use std::env::Args;

pub struct RunOptions {
    pub exit: bool,
    pub new: Option<String>,
    pub import: Option<String>,
    pub image: Option<String>
}

#[inline(always)]
pub fn parse_args(mut args: Args) -> Result<RunOptions, String> {
    let mut run_options = RunOptions {
        exit: false,
        new: None,
        import: None,
        image: None
    };

    if args.len() == 0 {
        print_usage();

        run_options.exit = true;
        return Ok(run_options);
    }

    while let Some(arg) = args.next() {
        if arg.eq("-v") || arg.eq("--version") {
            print_version();

            run_options.exit = true;
            return Ok(run_options);
        } else if arg.eq("-n") || arg.eq("--new") {
            run_options.new = match args.next() {
                Some(t) => Some(t),
                None => {
                    print_usage();

                    run_options.exit = true;
                    return Ok(run_options);
                }
            };

            continue;

        } else if arg.eq("-I") || arg.eq("--import") {
            run_options.import = match args.next() {
                Some(t) => Some(t),
                None => {
                    print_usage();

                    run_options.exit = true;
                    return Ok(run_options);
                }
            };

            continue;
        } else if arg.eq("-i") || arg.eq("--image") {
            run_options.image = match args.next() {
                Some(t) => Some(t),
                None => {
                    print_usage();

                    run_options.exit = true;
                    return Ok(run_options);
                }
            };

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
