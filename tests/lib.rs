use raytracer::{
    closest_intersection, image_to_file, AlmostEqual, Camera, Color, Image, Intersection, Radians,
    Ray, Sphere, UnitVector, Vector,
};
use std::str;

// Modified version of the original assert_eq implementation from
// https://github.com/rust-lang/rust/blob/909f5a049415a815b23502a5498de9a99bbf276b/src/libcore/macros.rs
macro_rules! assert_almost_eq {
    ($left:expr, $right:expr) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !((*left_val).almost_equal(right_val)) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    panic!(r#"assertion failed: `(left.almost_equal(right))`
  left: `{:?}`,
 right: `{:?}`"#, &*left_val, &*right_val)
                }
            }
        }
    });
    ($left:expr, $right:expr,) => ({
        assert_almost_eq!($left, $right)
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if !((*left_val).almost_equal(right_val)) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    panic!(r#"assertion failed: `(left.almost_equal(right))`
  left: `{:?}`,
 right: `{:?}`: {}"#, &*left_val, &*right_val,
                           format_args!($($arg)+))
                }
            }
        }
    });
}

#[test]
fn test_add() {
    assert_eq!(3, 3);
}

#[test]
fn test_vector_addition() {
    assert_almost_eq!(
        Vector {
            x: 1.0,
            y: 1.0,
            z: 1.0
        } + Vector {
            x: 1.0,
            y: 2.0,
            z: 3.0
        },
        Vector {
            x: 2.0,
            y: 3.0,
            z: 4.0
        }
    );
}

#[test]
fn test_vector_subtraction() {
    assert_almost_eq!(
        Vector {
            x: 5.0,
            y: 5.0,
            z: 5.0
        } - Vector {
            x: 1.0,
            y: 2.0,
            z: 3.0
        },
        Vector {
            x: 4.0,
            y: 3.0,
            z: 2.0
        }
    );
}

#[test]
fn test_vector_scalar_multiplication() {
    let initial_vector = Vector {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    let expected_vector = Vector {
        x: 2.0,
        y: 4.0,
        z: 6.0,
    };
    assert_almost_eq!(initial_vector * 2.0, expected_vector);
    assert_almost_eq!(2.0 * initial_vector, expected_vector);
}

#[test]
fn test_vector_scalar_division() {
    assert_almost_eq!(
        Vector {
            x: 1.0,
            y: 2.0,
            z: 3.0
        } / 2.0,
        Vector {
            x: 0.5,
            y: 1.0,
            z: 1.5
        }
    );
}

#[test]
fn test_vector_dot_product() {
    assert_almost_eq!(
        Vector {
            x: 1.0,
            y: 0.0,
            z: 0.0
        }
        .dot(&Vector {
            x: 0.0,
            y: 1.0,
            z: 1.0
        }),
        0.0
    );

    assert_almost_eq!(
        Vector {
            x: 1.0,
            y: 0.0,
            z: 0.0
        }
        .dot(&Vector {
            x: 1.0,
            y: 0.0,
            z: 0.0
        }),
        1.0
    );
}

#[test]
fn test_vector_cross_product() {
    let va = Vector {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    };
    let vb = Vector {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    assert_almost_eq!(
        va.cross(&vb),
        Vector {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        }
    );
}

#[test]
fn test_vector_normalization() {
    let original = Vector {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    assert_almost_eq!(
        original.normalized().0,
        Vector {
            x: 0.2672612419124244,
            y: 0.5345224838248488,
            z: 0.8017837257372732,
        }
    );
}

#[test]
fn test_sphere_ray_intersection() {
    let sphere = Sphere {
        center: Vector::zero(),
        radius: 1.0,
        color: Color::new_black(),
    };

    let outside_pointing_away = Ray {
        pos: Vector {
            x: 0.0,
            y: 0.0,
            z: 10.0,
        },
        dir: UnitVector(Vector {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        }),
    };
    assert_almost_eq!(
        sphere.intersect_ray(&outside_pointing_away),
        Intersection::None,
    );

    let outside_pointing_towards = Ray {
        pos: Vector {
            x: 0.0,
            y: 0.0,
            z: 10.0,
        },
        dir: UnitVector(Vector {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        }),
    };
    assert_almost_eq!(
        sphere.intersect_ray(&outside_pointing_towards),
        Intersection::Hit {
            position: Vector {
                x: 0.0,
                y: 0.0,
                z: 1.0
            },
            normal: Vector::unitz(),
            sphere: &sphere,
        }
    );

    let inside = Ray {
        pos: Vector::zero(),
        dir: UnitVector(Vector {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        }),
    };
    assert_almost_eq!(sphere.intersect_ray(&inside), Intersection::None);
}

#[test]
fn test_camera_screen_ray() {
    let camera = Camera {
        position: Vector::zero(),
        forward: -Vector::unitz(),
        up: Vector::unity(),
        aspect_ratio: 2.0 / 1.0,
        fovx: Radians(90.0f32.to_radians()),
    };

    assert_almost_eq!(
        camera.screen_ray(0.0, 0.0),
        Ray {
            pos: Vector::zero(),
            dir: Vector {
                x: -1.0,
                y: 0.5,
                z: -1.0,
            }
            .normalized(),
        }
    );

    assert_almost_eq!(
        camera.screen_ray(0.5, 0.5),
        Ray {
            pos: Vector::zero(),
            dir: -Vector::unitz(),
        },
    );

    assert_almost_eq!(
        camera.screen_ray(0.25, 0.25),
        Ray {
            pos: Vector::zero(),
            dir: Vector {
                x: -0.5,
                y: 0.25,
                z: -1.0,
            }
            .normalized(),
        }
    );
}

#[test]
fn test_unitvector_reflection() {
    assert_almost_eq!(
        Vector {
            x: -1.0,
            y: -1.0,
            z: -1.0,
        }
        .normalized()
        .reflected(
            &Vector {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            }
            .normalized()
        ),
        Vector {
            x: -1.0,
            y: 1.0,
            z: -1.0,
        }
        .normalized()
    );
}

#[test]
fn test_closest_intersection() {
    let spheres = [
        Sphere {
            center: Vector::zero(),
            radius: 1.0,
            color: Color::new_black(),
        },
        Sphere {
            center: Vector {
                x: 10.0,
                y: 0.0,
                z: 0.0,
            },
            radius: 1.0,
            color: Color::new_black(),
        },
    ];
    assert_almost_eq!(
        closest_intersection(
            &spheres,
            &Ray {
                pos: Vector {
                    x: -100.0,
                    y: 0.0,
                    z: 0.0,
                },
                dir: Vector::unitx(),
            }
        ),
        Intersection::Hit {
            position: Vector {
                x: -1.0,
                y: 0.0,
                z: 0.0,
            },
            normal: -Vector::unitx(),
            sphere: &spheres[0],
        },
    );

    assert_almost_eq!(
        closest_intersection(
            &spheres,
            &Ray {
                pos: Vector {
                    x: 100.0,
                    y: 0.0,
                    z: 0.0,
                },
                dir: -Vector::unitx(),
            },
        ),
        Intersection::Hit {
            position: Vector {
                x: 11.0,
                y: 0.0,
                z: 0.0,
            },
            normal: Vector::unitx(),
            sphere: &spheres[1],
        }
    );

    assert_almost_eq!(
        closest_intersection(
            &spheres,
            &Ray {
                pos: Vector {
                    x: 100.0,
                    y: 0.0,
                    z: 0.0
                },
                dir: Vector::unitx()
            },
        ),
        Intersection::None,
    );
}

#[test]
fn test_image_to_file() {
    let mut image = Image::new(3, 2);
    image.set_color(0, 0, Color::new_red());
    image.set_color(2, 1, Color::new_white());
    let mut buffer = Vec::new();
    image_to_file(&image, &mut buffer);
    let got = str::from_utf8(&buffer).unwrap();
    let expected = "P3
3 2
255
255 0 0 0 0 0 0 0 0 
0 0 0 0 0 0 255 255 255 
";
    assert_eq!(got, expected);
}

#[test]
fn test_color_scalar_multiplication() {
    assert_almost_eq!(
        Color {
            r: 1.0,
            g: 1.0,
            b: 1.0
        } * 0.5,
        Color {
            r: 0.5,
            g: 0.5,
            b: 0.5
        }
    );
    assert_almost_eq!(
        0.5 * Color {
            r: 1.0,
            g: 1.0,
            b: 1.0
        },
        Color {
            r: 0.5,
            g: 0.5,
            b: 0.5
        }
    );
}
