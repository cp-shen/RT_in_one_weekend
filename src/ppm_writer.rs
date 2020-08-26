use std::path::Path;
// use std::fs::File;
use std::fs;
use std::io::Write;

#[cfg(windows)]
const NL: &'static str = "\r\n";
#[cfg(not(windows))]
const NL: &'static str = "\n";

#[allow(dead_code)]
fn write_ppm(w: usize, h: usize, colors: &[u8], path: &Path) -> std::io::Result<()> {
    assert!(colors.len() == w * h * 3, "len of colors is not correct");

    let mut f = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .expect(&*format!("fail to open file {}", path.to_str().unwrap()));

    let mut content = format!("{}{}{} {}", "P3", NL, w, h);
    let pix_count = colors.len() / 3;

    todo!(); // use iter and range to write colors

    write!(f, "{}", content).unwrap();
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::ppm_writer::write_ppm;
    use std::path::Path;

    #[test]
    fn test() {
        let colors = [255_u8, 255_u8, 255_u8];
        let res = write_ppm(2, 1, &colors, Path::new("test.ppm"));
        assert!(res.is_ok())
    }
}

