use std::fs;
use std::io::Write;
use std::path::Path;

#[cfg(windows)]
const NL: &'static str = "\r\n";
#[cfg(not(windows))]
const NL: &'static str = "\n";

struct Pixel {
    x: u32,
    y: u32,
    color: crate::vec3::Color,
}

#[allow(dead_code)]
pub fn write_png(
    w: usize,
    h: usize,
    colors: &[u8],
    path: &Path,
) -> std::io::Result<()> {
    use image::{Rgb, RgbImage};
    let mut img = RgbImage::new(w as u32, h as u32);
    for row_idx in 0..h {
        for col_idx in 0..w {
            let r = colors[(row_idx * w + col_idx) * 3 + 0];
            let g = colors[(row_idx * w + col_idx) * 3 + 1];
            let b = colors[(row_idx * w + col_idx) * 3 + 2];
            let color = Rgb([r, g, b]);
            img.put_pixel(col_idx as u32, row_idx as u32, color);
        }
    }
    img.save(path).unwrap();
    Ok(())
}

#[allow(dead_code)]
pub fn write_ppm(
    w: usize,
    h: usize,
    colors: &[u8],
    path: &Path,
) -> std::io::Result<()> {
    assert!(
        colors.len() == w * h * 3,
        "len of colors {} is not equal to w:{} * h:{} * 3",
        colors.len(),
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
    content = colors
        .iter()
        .enumerate()
        .map(|(i, color)| {
            let mut color_str = String::new();
            if i % (3 * w) == 0 {
                color_str.push_str(NL);
            }
            color_str.push_str(format!("{}\t", color).as_str());
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
    use crate::image_writer::write_ppm;
    use std::path::Path;

    #[test]
    fn test_simple_ppm() {
        let yellow = [255_u8, 255_u8, 0_u8];
        let white = [255_u8, 255_u8, 255_u8];
        let mut colors_vec = Vec::<u8>::new();
        for x in 0..9 {
            colors_vec.push(yellow[x % 3]);
        }
        for x in 0..9 {
            colors_vec.push(white[x % 3]);
        }
        let res =
            write_ppm(3, 2, &colors_vec, Path::new("images/test_simle.ppm"));
        assert!(res.is_ok())
    }

    #[test]
    fn test_gradient() {
        let image_width = 256;
        let image_height = 256;
        let color_vec = Vec::<u8>::new();

        (image_width).map(|x|{(0..image_height)});
    }
}
