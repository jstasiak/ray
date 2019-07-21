use crate::material::Color;
use std::io::Write;

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

#[cfg(test)]
mod tests {
    use crate::image::{image_to_file, Image};
    use crate::material::Color;
    use std::str;

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
}
