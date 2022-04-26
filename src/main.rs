use std::io::{self, Write};

fn main() {
    let _ = make_ppm();
}

fn make_ppm() -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let image_width: i32 = 256;
    let image_height: i32 = 256;

    handle.write_fmt(format_args!("P3\n{} {}\n255\n", image_width, image_height))?;

    let mut j = image_height - 1;
    while j >= 0 {
        let mut i = 0;
        while i < image_width {
            let r = i as f32 / (image_width - 1) as f32;
            let g = j as f32 / (image_height - 1) as f32;
            let b = 0.25;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            handle.write_fmt(format_args!("{} {} {}\n", ir, ig, ib))?;

            i += 1;
        }

        j -= 1;
    }

    Ok(())
}
