use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone, Debug)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    pub fn almost_equal(&self, other: &Vector) -> bool {
        self.almost_equal_with_epsilon(other, 0.00000001)
    }

    pub fn almost_equal_with_epsilon(&self, other: &Vector, epsilon: f32) -> bool {
        (self.x - other.x).abs() < epsilon
            && (self.y - other.y).abs() < epsilon
            && (self.z - other.z).abs() < epsilon
    }

    pub fn zero() -> Vector {
        Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn dot(&self, other: &Vector) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vector) -> Vector {
        Vector {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn len(&self) -> f32 {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt()
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        self + -1.0 * other
    }
}

impl Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, other: f32) -> Vector {
        Vector {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Mul<Vector> for f32 {
    type Output = Vector;

    fn mul(self, other: Vector) -> Vector {
        other * self
    }
}

impl Div<f32> for Vector {
    type Output = Vector;

    fn div(self, other: f32) -> Vector {
        Vector {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Ray {
    pub pos: Vector,
    pub dir: Vector,
}

impl Ray {
    fn forwarded(&self, distance: f32) -> Ray {
        Ray {
            pos: self.pos + self.dir * distance,
            dir: self.dir,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Sphere {
    pub center: Vector,
    pub radius: f32,
}

impl Sphere {
    pub fn intersect_ray(&self, ray: &Ray) -> Intersection {
        // Math based on information found on
        // http://kylehalladay.com/blog/tutorial/math/2013/12/24/Ray-Sphere-Intersection.html
        //
        let pos_to_center = self.center - ray.pos;
        // No support for intersections with rays coming from inside the sphere at the moment.
        if pos_to_center.len() <= self.radius {
            return Intersection::None;
        }
        // tcenter is how far along the ray dir we need to go in order for the line orthogonal to
        // the ray to cross the sphere's center. Let's call that point on the ray C.
        let tcenter = pos_to_center.dot(&ray.dir);
        // The sphere is in the opposite direction.
        if tcenter < 0.0 {
            return Intersection::None;
        }
        // We now have a right triangle with [ray.pos C] being one of its leg and [ray.pos
        // sphere.center] being its hypotenuse. The distance between C and self.center is what we
        // need to find out and its the remaining leg of the triangle – let's use the Pythagorean
        // theorem. We'll call the [C self.center] distance d.
        let d = (pos_to_center.len().powf(2.0) - tcenter.powf(2.0)).sqrt();
        // If we miss the sphere totally the distance d will be greater than the radius, let's bail
        // in that case.
        if d > self.radius {
            return Intersection::None;
        }
        // Now we have two right triangles with self.radius being its hypotenuse and d forming one
        // of its legs. The remaining leg is a distance tdelta that we'll use to move forward and
        // backward along the ray starting with point C in order to get two points at which we
        // intersect the sphere. Again – just Pythagorean theorem at work here.
        let tdelta = (self.radius.powf(2.0) - d.powf(2.0)).sqrt();
        // We can now calculate two points at which we cross the sphere, but we only need the
        // closer one so let's do just that.
        let intersection_point = ray.forwarded(tcenter - tdelta).pos;
        Intersection::Hit(intersection_point)
    }
}

#[derive(Debug)]
pub enum Intersection {
    None,
    Hit(Vector),
}

impl Intersection {
    pub fn almost_equal(&self, other: &Intersection) -> bool {
        match self {
            Intersection::None => match other {
                Intersection::None => true,
                _ => false,
            },
            Intersection::Hit(v1) => match other {
                Intersection::Hit(v2) => v1.almost_equal(&v2),
                _ => false,
            },
        }
    }
}

pub fn almost_equal(a: f32, b: f32) -> bool {
    almost_equal_with_epsilon(a, b, 0.00000001)
}

pub fn almost_equal_with_epsilon(a: f32, b: f32, epsilon: f32) -> bool {
    (a - b).abs() < epsilon
}
