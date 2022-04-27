use std::io::{self, stderr, stdout, Write};

mod vec3;

fn main() {
    let _ = make_ppm();

    let v1 = vec3::Vec3::new(0.0, 0.0, 0.);

    let _ = v1.x();
    let _ = v1.y();
    let _ = v1.z();

    let _ = v1[1];
}

fn make_ppm() -> io::Result<()> {
    let stdout = stdout();
    let stderr = stderr();
    let mut stdout = stdout.lock();
    let mut stderr = stderr.lock();

    let image_width: i32 = 256;
    let image_height: i32 = 256;

    stdout.write_fmt(format_args!("P3\n{} {}\n255\n", image_width, image_height))?;

    let mut j = image_height - 1;
    while j >= 0 {
        stderr.write_fmt(format_args!("\rScanlines remaining: {}", j))?;
        stderr.flush()?;

        let mut i = 0;
        while i < image_width {
            let r = i as f32 / (image_width - 1) as f32;
            let g = j as f32 / (image_height - 1) as f32;
            let b = 0.25;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            stdout.write_fmt(format_args!("{} {} {}\n", ir, ig, ib))?;

            i += 1;
        }

        j -= 1;
    }

    stderr.write_all(b"\nDone.\n")?;

    Ok(())
}
