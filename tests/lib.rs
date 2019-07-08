use raytracer::{almost_equal, Intersection, Ray, Sphere, Vector};

#[test]
fn test_add() {
    assert_eq!(3, 3);
}

#[test]
fn test_vector_addition() {
    assert!((Vector {
        x: 1.0,
        y: 1.0,
        z: 1.0
    } + Vector {
        x: 1.0,
        y: 2.0,
        z: 3.0
    })
    .almost_equal(&Vector {
        x: 2.0,
        y: 3.0,
        z: 4.0
    }));
}

#[test]
fn test_vector_subtraction() {
    assert!((Vector {
        x: 5.0,
        y: 5.0,
        z: 5.0
    } - Vector {
        x: 1.0,
        y: 2.0,
        z: 3.0
    })
    .almost_equal(&Vector {
        x: 4.0,
        y: 3.0,
        z: 2.0
    }));
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
    assert!((initial_vector * 2.0).almost_equal(&expected_vector));
    assert!((2.0 * initial_vector).almost_equal(&expected_vector));
}

#[test]
fn test_vector_scalar_division() {
    assert!((Vector {
        x: 1.0,
        y: 2.0,
        z: 3.0
    } / 2.0)
        .almost_equal(&Vector {
            x: 0.5,
            y: 1.0,
            z: 1.5
        }));
}

#[test]
fn test_vector_dot_product() {
    assert!(almost_equal(
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
    ));

    assert!(almost_equal(
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
    ));
}

#[test]
fn test_sphere_ray_intersection() {
    let sphere = Sphere {
        center: Vector::zero(),
        radius: 1.0,
    };

    let outside_pointing_away = Ray {
        pos: Vector {
            x: 0.0,
            y: 0.0,
            z: 10.0,
        },
        dir: Vector {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
    };
    let intersection1 = sphere.intersect_ray(&outside_pointing_away);
    assert!(
        intersection1.almost_equal(&Intersection::None),
        "Got: {:?}",
        intersection1
    );

    let outside_pointing_towards = Ray {
        pos: Vector {
            x: 0.0,
            y: 0.0,
            z: 10.0,
        },
        dir: Vector {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
    };
    let intersection2 = sphere.intersect_ray(&outside_pointing_towards);
    assert!(
        intersection2.almost_equal(&Intersection::Hit(Vector {
            x: 0.0,
            y: 0.0,
            z: 1.0
        })),
        "Got: {:?}",
        intersection2
    );

    let inside = Ray {
        pos: Vector::zero(),
        dir: Vector {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        },
    };
    let intersection3 = sphere.intersect_ray(&inside);
    assert!(
        intersection3.almost_equal(&Intersection::None),
        "Got: {:?}",
        intersection3
    );
}
