use crate::traits::AlmostEqual;
use std::f32;
use std::ops::{Add, Mul};

#[derive(Copy, Clone, Debug)]
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

    pub fn new_green() -> Color {
        Self::new(0.0, 1.0, 0.0)
    }

    pub fn new_blue() -> Color {
        Self::new(0.0, 0.0, 1.0)
    }

    pub fn new_white() -> Color {
        Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        }
    }
}

impl AlmostEqual for Color {
    fn almost_equal(&self, other: &Color) -> bool {
        self.r.almost_equal(&other.r)
            && self.g.almost_equal(&other.g)
            && self.b.almost_equal(&other.b)
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        // Adding two Colors together clamps the r, g and b values to [0.0, 1.0] range
        Color {
            r: clamp(0.0, 1.0, self.r + other.r),
            g: clamp(0.0, 1.0, self.g + other.g),
            b: clamp(0.0, 1.0, self.b + other.b),
        }
    }
}

fn clamp(min: f32, max: f32, value: f32) -> f32 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, other: f32) -> Color {
        // The floating point operand needs to be strictly within (0.0, 1.0) range, this is for
        // simple scaling. May revisit later to do multiplication by values larger than 1.0 and
        // clamping afterwards.
        assert!(0.0 <= other && other <= 1.0);
        Color {
            r: self.r * other,
            g: self.g * other,
            b: self.b * other,
        }
    }
}

impl Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        // See impl Mul<f32> for Color comment.
        assert!(0.0 <= self && self <= 1.0);
        other * self
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_almost_eq;
    use crate::material::Color;
    use crate::traits::AlmostEqual;

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

    #[test]
    fn test_color_addition() {
        assert_almost_eq!(
            Color::new_red() + Color::new_green() + Color::new_blue(),
            Color::new_white()
        );
        assert_almost_eq!(Color::new_red() + Color::new_red(), Color::new_red());
    }
}
