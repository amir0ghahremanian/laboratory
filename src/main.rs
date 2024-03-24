mod cmd;
mod image;

use std::env::args;

use cmd::parse_args;
use image::Lab;

fn main() -> Result<(), String> {
    let mut args = args();
    args.next();

    let run_options = parse_args(args)?;

    if run_options.exit {
        return Ok(());
    }

    Lab::from_image("test".to_string()).expand("test".to_string())
}
