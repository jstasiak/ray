use crate::image::Image;
use crate::scene::{trace_ray, Camera, Sphere};

pub fn render(
    spheres: &[Sphere],
    camera: &Camera,
    width: usize,
    height: usize,
    bounces: usize,
) -> Image {
    let mut image = Image::new(width, height);
    let pixels_total = width * height;
    let mut pixels_done = 0;
    let mut percent = 0;
    for i in 0..width {
        for j in 0..height {
            // -1s here because we want to provide x and y coordinates between 0 and 1 inclusive
            let ray = camera.screen_ray(
                i as f32 / (width - 1) as f32,
                j as f32 / (height - 1) as f32,
            );
            let color = trace_ray(&spheres, &ray, bounces);
            image.set_color(i, j, color);
            pixels_done += 1;
            let new_percent = pixels_done * 100 / pixels_total;
            if new_percent != percent {
                eprintln!("{}% done...", new_percent);
                percent = new_percent;
            }
        }
    }
    image
}
