use std::io::{self, stderr, stdout, Write};

// declare the modules
mod color;
mod point3;
mod ray;
mod vec3;

use point3::Point3;
use ray::Ray;
use vec3::Vec3;

fn main() {
    let _ = make_ppm();
}

fn ray_color(r: &ray::Ray) -> color::Color {
    let unit_dir = vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_dir.y() + 1.0);

    let c1 = color::Color::new(1.0, 1.0, 1.0);
    let c2 = color::Color::new(0.5, 0.7, 1.0);

    (1.0 - t) * c1 + t * c2
}

fn make_ppm() -> io::Result<()> {
    let stdout = stdout();
    let stderr = stderr();
    let mut stdout = stdout.lock();
    let mut stderr = stderr.lock();

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: i32 = 400;
    let image_height: i32 = (image_width as f32 / aspect_ratio) as i32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Render

    stdout.write_fmt(format_args!("P3\n{} {}\n255\n", image_width, image_height))?;

    let mut j = image_height - 1;
    while j >= 0 {
        stderr.write_fmt(format_args!("\rScanlines remaining: {}", j))?;
        stderr.flush()?;

        let mut i = 0;
        while i < image_width {
            let u = i as f32 / (image_width - 1) as f32;
            let v = j as f32 / (image_height - 1) as f32;

            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );

            let pixel_color = ray_color(&r);
            color::write_color(&mut stdout, pixel_color);

            i += 1;
        }

        j -= 1;
    }

    stderr.write_all(b"\nDone.\n")?;

    Ok(())
}
