use raytracer::{image_to_file, render};
use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!(
            "Usage: {} <output filename> (<output filename> may be - for stdout)",
            args[0],
        );
        process::exit(1);
    }

    let filename = &args[1];
    let mut file: Box<Write> = match filename.as_ref() {
        "-" => Box::new(io::stdout()),
        _ => Box::new(File::create(filename).expect("Cannot open file for writing")),
    };

    let image = render(200, 100);
    image_to_file(&image, &mut file);
}
