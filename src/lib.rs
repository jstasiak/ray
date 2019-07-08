use std::io::Write;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Copy, Clone, Debug)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    pub fn almost_equal(&self, other: &Vector) -> bool {
        self.almost_equal_with_epsilon(other, 0.0000001)
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

    pub fn unitx() -> UnitVector {
        UnitVector(Vector {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        })
    }

    pub fn unity() -> UnitVector {
        UnitVector(Vector {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        })
    }

    pub fn unitz() -> UnitVector {
        UnitVector(Vector {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        })
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

    pub fn normalized(&self) -> UnitVector {
        let len = self.len();
        UnitVector(Vector {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        })
    }
    pub fn is_normalized(&self) -> bool {
        almost_equal(self.len(), 1.0)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct UnitVector(pub Vector);

impl Neg for UnitVector {
    type Output = UnitVector;

    fn neg(self) -> UnitVector {
        UnitVector(-self.0)
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

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub pos: Vector,
    pub dir: UnitVector,
}

impl Ray {
    fn forwarded(&self, distance: f32) -> Ray {
        Ray {
            pos: self.pos + self.dir.0 * distance,
            dir: self.dir,
        }
    }

    pub fn almost_equal(&self, other: &Ray) -> bool {
        self.pos.almost_equal(&other.pos) && self.dir.0.almost_equal(&other.dir.0)
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
        let tcenter = pos_to_center.dot(&ray.dir.0);
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
        Intersection::Hit(IntersectionData {
            position: intersection_point,
            normal: (intersection_point - self.center).normalized(),
        })
    }
}

#[derive(Debug)]
pub enum Intersection {
    None,
    Hit(IntersectionData),
}

impl Intersection {
    pub fn almost_equal(&self, other: &Intersection) -> bool {
        match self {
            Intersection::None => match other {
                Intersection::None => true,
                _ => false,
            },
            Intersection::Hit(d1) => match other {
                Intersection::Hit(d2) => d1.almost_equal(&d2),
                _ => false,
            },
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct IntersectionData {
    pub position: Vector,
    pub normal: UnitVector,
}

impl IntersectionData {
    pub fn almost_equal(&self, other: &IntersectionData) -> bool {
        self.position.almost_equal(&other.position) && self.normal.0.almost_equal(&other.normal.0)
    }
}

pub fn almost_equal(a: f32, b: f32) -> bool {
    almost_equal_with_epsilon(a, b, 0.0000001)
}

pub fn almost_equal_with_epsilon(a: f32, b: f32, epsilon: f32) -> bool {
    (a - b).abs() < epsilon
}

pub struct Camera {
    pub position: Vector,
    // The forward and up vectors have to be normalized
    pub forward: UnitVector,
    pub up: UnitVector,
    pub aspect_ratio: f32,
    pub fovx: Radians,
}

impl Camera {
    pub fn screen_ray(&self, x: f32, y: f32) -> Ray {
        // We assume that a screen lies 1 unit in front of the camera. The center (x: 0.5, y: 0.5) of the screen
        // lies directly on the forward axis.
        assert!(0.0 <= x && x <= 1.0);
        assert!(0.0 <= y && y <= 1.0);
        let right = self.forward.0.cross(&self.up.0);
        // top left corner is x -1.0, y 1.0
        let xunit = posunit_to_unit(x);
        let yunit = -posunit_to_unit(y);
        // The distance between a point on the screen and the center of the screen forms a right
        // triangle with the distance between the camera and the center of the screen and the
        // distance between the camera and the point. Since we know the maximum angle we can go in
        // either direction (fovx/2 for x, fovy/2 for y) we first calculate the size of the screen
        // 1 unit in front of the camera using tangent:
        let screen_width = 2.0 * (self.fovx.0 / 2.0).tan();
        let screen_height = screen_width / self.aspect_ratio;
        // What's left now is to calculate the point at the screen we're looking at and a ray
        // pointing to it:
        let point_at_screen = self.position
            + self.forward.0
            + right * xunit * screen_width / 2.0
            + self.up.0 * yunit * screen_height / 2.0;
        let ray = Ray {
            pos: self.position,
            dir: (point_at_screen - self.position).normalized(),
        };
        ray
    }
}

pub fn posunit_to_unit(value: f32) -> f32 {
    // Convert value in range [0.0, 1.0] to value in range [-1.0, 1.0]
    value * 2.0 - 1.0
}

pub struct Radians(pub f32);

pub fn render(spheres: &[Sphere], camera: &Camera, width: usize, height: usize) -> Image {
    let mut image = Image::new(width, height);
    for i in 0..width {
        for j in 0..height {
            // -1s here because we want to provide x and y coordinates between 0 and 1 inclusive
            let ray = camera.screen_ray(
                i as f32 / (width - 1) as f32,
                j as f32 / (height - 1) as f32,
            );
            let intersection = closest_intersection(&spheres, &ray);
            let color = match intersection {
                Intersection::None => Color::new_black(),
                Intersection::Hit(IntersectionData { normal, .. }) => {
                    let brightness = 1.0 - normal.0.dot(&ray.dir.0);
                    Color {
                        r: brightness,
                        g: brightness,
                        b: brightness,
                    }
                }
            };
            image.set_color(i, j, color);
        }
    }
    image
}

pub fn closest_intersection(spheres: &[Sphere], ray: &Ray) -> Intersection {
    let mut hits = Vec::new();
    for sphere in spheres {
        match sphere.intersect_ray(&ray) {
            Intersection::Hit(point) => hits.push(point),
            Intersection::None => (),
        }
    }
    if hits.len() == 0 {
        return Intersection::None;
    }
    let mut closest = hits[0];
    let mut closest_distance = (closest.position - ray.pos).len();
    for hit in hits {
        let distance = (hit.position - ray.pos).len();
        if distance < closest_distance {
            closest_distance = distance;
            closest = hit;
        }
    }
    Intersection::Hit(closest)
}

pub fn image_to_file(image: &Image, w: &mut Write) {
    write!(w, "P3\n{} {}\n255\n", image.width(), image.height()).expect("Cannot write");

    for y in 0..image.height() {
        for x in 0..image.width() {
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

pub struct Image {
    buffer: Vec<Color>,
    w: usize,
    h: usize,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Image {
        Image {
            buffer: vec![Color::new_black(); width * height],
            w: width,
            h: height,
        }
    }

    pub fn set_color(&mut self, x: usize, y: usize, color: Color) {
        self.buffer[y * self.w + x] = color;
    }

    pub fn get_color(&self, x: usize, y: usize) -> Color {
        self.buffer[y * self.w + x]
    }

    pub fn width(&self) -> usize {
        self.w
    }

    pub fn height(&self) -> usize {
        self.h
    }
}

#[derive(Copy, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color { r, g, b }
    }

    pub fn new_black() -> Color {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn new_red() -> Color {
        Self::new(1.0, 0.0, 0.0)
    }
}
