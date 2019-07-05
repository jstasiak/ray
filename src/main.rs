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

fn render(width: usize, height: usize) -> Image {
    let mut image = Image::new(width, height);
    let max = (width * height) as f32;
    for i in 0..width {
        for j in 0..height {
            image.set_color(
                i,
                j,
                Color::new(
                    i as f32 / width as f32,
                    j as f32 / height as f32,
                    i as f32 * j as f32 / max,
                ),
            );
        }
    }
    image
}

fn image_to_file(image: &Image, w: &mut Write) {
    write!(w, "P3\n{} {}\n255\n", image.width(), image.height()).expect("Cannot write");

    for x in 0..image.width() {
        for y in 0..image.height() {
            let color = image.get_color(x, y);
            write!(
                w,
                "{} {} {} ",
                (color.r * 255.0) as u8,
                (color.g * 255.0) as u8,
                (color.b * 255.0) as u8
            )
            .expect("Cannot write");
        }
        write!(w, "\n").expect("Cannot write");
    }
    w.flush().expect("Cannot flush");
}

struct Image {
    buffer: Vec<Color>,
    w: usize,
    h: usize,
}

impl Image {
    fn new(width: usize, height: usize) -> Image {
        Image {
            buffer: vec![Color::new_black(); width * height],
            w: width,
            h: height,
        }
    }

    fn set_color(&mut self, x: usize, y: usize, color: Color) {
        self.buffer[y * self.w + x] = color;
    }

    fn get_color(&self, x: usize, y: usize) -> Color {
        self.buffer[y * self.w + x]
    }

    fn width(&self) -> usize {
        self.w
    }

    fn height(&self) -> usize {
        self.h
    }
}

#[derive(Copy, Clone)]
struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    fn new(r: f32, g: f32, b: f32) -> Color {
        Color { r, g, b }
    }

    fn new_black() -> Color {
        Self::new(0.0, 0.0, 0.0)
    }
}
