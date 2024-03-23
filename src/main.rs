mod image;

use std::env::args;

use image::Lab;

fn main() -> Result<(), &'static str> {
    let mut args = args();
    args.next();

    Lab::from_image("path".to_string()).expand("path".to_string())
}
