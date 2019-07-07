use raytracer::Vector;

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
