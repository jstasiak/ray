use raytracer::{image_to_file, render, Camera, Radians, Sphere, Vector};
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

    let spheres = [
        Sphere {
            center: Vector {
                x: 0.0,
                y: 0.0,
                z: -5.0,
            },
            radius: 1.0,
        },
        Sphere {
            center: Vector {
                x: -3.0,
                y: 1.0,
                z: -5.0,
            },
            radius: 1.0,
        },
        Sphere {
            center: Vector {
                x: 5.0,
                y: 1.0,
                z: -10.0,
            },
            radius: 1.0,
        },
    ];
    let camera = Camera {
        position: Vector::zero(),
        forward: -Vector::unitz(),
        up: Vector::unity(),
        aspect_ratio: 4.0 / 3.0,
        fovx: Radians(90.0f32.to_radians()),
    };
    let image = render(&spheres, &camera, 800, 600);
    image_to_file(&image, &mut file);
}
