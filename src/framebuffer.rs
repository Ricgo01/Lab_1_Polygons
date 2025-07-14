use raylib::prelude::*;

pub struct Framebuffer {
    width: i32,
    height: i32,
    pixels: Vec<Color>,
    current_color: Color,
}

impl Framebuffer {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            pixels: vec![Color::BLACK; (width * height) as usize],
            current_color: Color::WHITE,
        }
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn set_pixel(&mut self, x: i32, y: i32) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            let index = (y * self.width + x) as usize;
            self.pixels[index] = self.current_color;
        }
    }

    pub fn render_to_file(&self, file_path: &str) {
        let mut image = Image::gen_image_color(self.width, self.height, Color::BLANK);
        for y in 0..self.height {
            for x in 0..self.width {
                let index = (y * self.width + x) as usize;
                image.draw_pixel(x, y, self.pixels[index]);
            }
        }
        image.export_image(file_path);
    }
}