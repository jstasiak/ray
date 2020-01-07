use ray::{image_to_file, render, Camera, Color, Material, Radians, Sphere, Vector};
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
            material: Material {
                color: Color::new_red(),
            },
        },
        Sphere {
            center: Vector {
                x: -3.0,
                y: 1.0,
                z: -5.0,
            },
            radius: 1.0,
            material: Material {
                color: Color::new_green(),
            },
        },
        Sphere {
            center: Vector {
                x: 5.0,
                y: 1.0,
                z: -10.0,
            },
            radius: 1.0,
            material: Material {
                color: Color::new_blue(),
            },
        },
        // Let's simulate walls, floor and ceiling with spheres
        Sphere {
            center: Vector {
                x: 0.0,
                y: -10005.0,
                z: 0.0,
            },
            radius: 10000.0,
            material: Material {
                color: Color::new_white(),
            },
        },
        Sphere {
            center: Vector {
                x: 0.0,
                y: 10005.0,
                z: 0.0,
            },
            radius: 10000.0,
            material: Material {
                color: Color::new_white(),
            },
        },
        Sphere {
            center: Vector {
                x: -10010.0,
                y: 0.0,
                z: 0.0,
            },
            radius: 10000.0,
            material: Material {
                color: Color::new_white(),
            },
        },
        Sphere {
            center: Vector {
                x: 10010.0,
                y: 0.0,
                z: 0.0,
            },
            radius: 10000.0,
            material: Material {
                color: Color::new_white(),
            },
        },
        Sphere {
            center: Vector {
                x: 0.0,
                y: 0.0,
                z: -10015.0,
            },
            radius: 10000.0,
            material: Material {
                color: Color::new_white(),
            },
        },
        Sphere {
            center: Vector {
                x: 0.0,
                y: 0.0,
                z: 10005.0,
            },
            radius: 10000.0,
            material: Material {
                color: Color::new_white(),
            },
        },
    ];
    let camera = Camera {
        position: Vector::zero(),
        forward: -Vector::unitz(),
        up: Vector::unity(),
        aspect_ratio: 4.0 / 3.0,
        fovx: Radians(90.0f32.to_radians()),
    };
    let image = render(&spheres, &camera, 800, 600, 3);
    image_to_file(&image, &mut file);
}
