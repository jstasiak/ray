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

    render(&mut file, 200, 100);
}

fn render(w: &mut Write, width: isize, height: isize) {
    write!(w, "P3\n{} {}\n255\n", width, height).expect("Cannot write");

    for _ in 0..width {
        for _ in 0..height {
            write!(w, "100 0 0 ").expect("Cannot write");
        }
        write!(w, "\n").expect("Cannot write");
    }
    w.flush().expect("Cannot flush");
}
