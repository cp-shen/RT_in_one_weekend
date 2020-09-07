use std::fs;
use std::io::Write;
use std::path::Path;

#[cfg(windows)]
const NL: &'static str = "\r\n";
#[cfg(not(windows))]
const NL: &'static str = "\n";

#[derive(Copy, Clone)]
pub struct Pixel {
    pub x: u32, //left is zero
    pub y: u32, //bottom is zero
    pub color: crate::vec3::Color,
}

#[allow(dead_code)]
pub fn write_png(
    w: u32,
    h: u32,
    pixels: &[Pixel],
    path: &Path,
) -> std::io::Result<()> {
    use image::{Rgb, RgbImage};
    let mut img = RgbImage::new(w as u32, h as u32);
    pixels.iter().for_each(|p| {
        assert!(p.x < w);
        assert!(p.y < h);
        let c = p.color.float_color_to_8bit();
        let color = Rgb([c.0, c.1, c.2]);
        img.put_pixel(p.x, h - p.y - 1, color);
    });
    img.save(path).unwrap();
    Ok(())
}

#[allow(dead_code)]
pub fn write_ppm(
    w: u32,
    h: u32,
    pixels: &[Pixel],
    path: &Path,
) -> std::io::Result<()> {
    assert!(
        pixels.len() as u32 == w * h,
        "len of pixels {} is not equal to w:{} * h:{}",
        pixels.len(),
        w,
        h
    );

    let mut f = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .expect(
            format!("fail to open file {}", path.to_str().unwrap()).as_str(),
        );

    let header = format!("{} {} {} {} {} {}", "P3", NL, w, h, NL, 255);

    let mut content = &mut String::new();
    let mut pixels_sorted = pixels.to_vec();
    pixels_sorted.sort_by(|a, b| {
        if a.y != b.y {
            return b.y.partial_cmp(&a.y).unwrap(); //reverse
        } else if a.x != b.x {
            return a.x.partial_cmp(&b.x).unwrap();
        } else {
            return std::cmp::Ordering::Equal;
        }
    });
    content = pixels_sorted
        .iter()
        .enumerate()
        .map(|(i, pixel)| {
            let mut color_str = String::new();
            if i as u32 % w == 0 {
                color_str.push_str(NL);
            }
            let color = pixel.color.float_color_to_8bit();
            color_str.push_str(
                format!("{}\t{}\t{}\t", color.0, color.1, color.2).as_str(),
            );
            color_str
        })
        .fold(content, |str, str_a| {
            str.push_str(str_a.as_str());
            str
        });

    write!(f, "{}{}", header.as_str(), content.as_str()).unwrap();
    f.flush().unwrap();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::Pixel;
    use crate::image_writer::*;
    use crate::vec3::*;
    use std::path::Path;

    #[test]
    fn test_simple_ppm() {
        let yellow = Color::new(1_f32, 1_f32, 0_f32);
        let white = Color::new(1_f32, 1_f32, 1_f32);

        let mut colors_vec = Vec::<Pixel>::new();
        for x in 0..100 {
            for y in 0..100 {
                let color = if y < 50_u32 { yellow } else { white };
                let p = Pixel { color, x, y };
                colors_vec.push(p);
            }
        }
        let res = write_ppm(
            100,
            100,
            &colors_vec,
            Path::new("images/test_simple.ppm"),
        );
        assert!(res.is_ok())
    }

    #[test]
    fn test_gradient() {
        let image_width: u32 = 256;
        let image_height: u32 = 256;
        let mut colors_vec = Vec::<Pixel>::new();

        for x in 0..image_width {
            for y in 0..image_height {
                let r = x as f32 / (image_width - 1) as f32;
                let g = y as f32 / (image_height - 1) as f32;
                let b = 0.25_f32;
                let color = Color::new(r, g, b);
                let p = Pixel { color, x, y };
                colors_vec.push(p);
            }
        }
        let res = write_png(
            image_width,
            image_height,
            &colors_vec,
            Path::new("images/test_gradient.png"),
        );
        assert!(res.is_ok())
    }
}
