use std::io::Write;

use crate::vec3;

pub type Color = vec3::Vec3;

pub fn write_color<W: Write>(mut stream: W, pixel_color: Color) {
    let x = (255.999 * pixel_color.x()) as i32;
    let y = (255.999 * pixel_color.y()) as i32;
    let z = (255.999 * pixel_color.z()) as i32;

    stream
        .write_fmt(format_args!("{} {} {}\n", x, y, z))
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::BufWriter;

    #[test]
    fn write_color() {
        let c = Color::new(0.1, 0.2, 0.3);

        let mut buffer = [0u8; 8];
        let writer = BufWriter::new(buffer.as_mut());
        super::write_color(writer, c);

        assert_eq!("25 51 76", String::from_utf8_lossy(&buffer))
    }
}
