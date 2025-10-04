use std::fs::File;
use std::io::{BufWriter, Write};
use image::ImageReader;

#[derive(Debug, Clone)]
pub struct Image {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Color>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height, pixels: Vec::with_capacity(width * height) }
    }

        pub fn save_ppm(&self, path: &str) -> std::io::Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        writeln!(writer, "P3")?;
        writeln!(writer, "{} {}", self.width, self.height)?;
        writeln!(writer, "255")?;

        for pixel in &self.pixels {
            writeln!(writer, "{}", pixel)?;
        }

        Ok(())
    }
}