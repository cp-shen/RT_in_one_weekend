use std::path::Path;
use std::fs;
use std::io::Write;

#[cfg(windows)]
const NL: &'static str = "\r\n";
#[cfg(not(windows))]
const NL: &'static str = "\n";

#[allow(dead_code)]
fn write_ppm(w: usize, h: usize, colors: &[u8], path: &Path) -> std::io::Result<()> {
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
        .open(path)
        .expect(format!("fail to open file {}", path.to_str().unwrap()).as_str());

    let header = format!("{} {} {} {}", "P3", NL, w, h);

    let mut content = &mut String::new();
    content = colors
        .iter()
        .enumerate()
        .map(|(i, color)| {
            let mut color_str = String::new();
            if i % 3 == 0 {
                color_str.push_str(NL);
            }
            color_str.push_str(format!("{} ", color).as_str());
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
    use crate::ppm_writer::write_ppm;
    use std::path::Path;

    #[test]
    fn test() {
        let colors = [255_u8, 255_u8, 0_u8]; // yellow
        let mut colors_vec = Vec::<u8>::new();
        for x in 0..9 {
            colors_vec.push(colors[x % 3]);
        }
        let res = write_ppm(3, 1, &colors_vec, Path::new("test.ppm"));
        assert!(res.is_ok())
    }
}
