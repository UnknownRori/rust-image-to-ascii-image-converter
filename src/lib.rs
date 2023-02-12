use image::{GenericImageView, ImageBuffer, ImageError, Rgba};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};

const BRIGHTNESS_SCALE: &str = " ,^-~:*+=\\[(&$@#";

#[derive(Debug)]
pub struct ASCIIConverter<'a> {
    src_image_dir: &'a str,  // Source image
    dest_image_dir: &'a str, // Destination image
    scale: i32, // Image scaling factor (higher values mean single pixel represent multiple pixels)
    font_size: u32, // Size of the font
}

impl<'a> ASCIIConverter<'a> {
    pub fn new(
        src_image_dir: &'a str,
        dest_image_dir: &'a str,
        font_size: u32,
        scale: i32,
    ) -> ASCIIConverter<'a> {
        ASCIIConverter {
            src_image_dir,
            dest_image_dir,
            font_size,
            scale,
        }
    }

    pub fn set_src(&mut self, src: &'a str) {
        self.src_image_dir = src;
    }

    pub fn set_dest(&mut self, dest: &'a str) {
        self.dest_image_dir = dest;
    }

    pub fn set_font_size(&mut self, font_size: u32) {
        self.font_size = font_size;
    }

    pub fn set_scale(&mut self, scale: i32) {
        self.scale = scale;
    }

    // Todo : Need rework!
    pub fn convert_to_img(&self) -> Result<(), ImageError> {
        let image = image::open(&self.src_image_dir)?;

        let (width, height) = image.dimensions();

        let new_width = width * self.font_size;
        let new_height = height * self.font_size;
        let mut img_ascii = ImageBuffer::new(new_width, new_height);
        let scale = Scale {
            x: self.font_size as f32,
            y: self.font_size as f32,
        };
        let font = include_bytes!("FiraCode-Regular.ttf");
        let font = Font::try_from_bytes(font).unwrap();

        for y in 0..height {
            for x in 0..width {
                let mut ascii = String::new();
                let pixel = image.get_pixel(x, y).0[0];
                let char_index =
                    (pixel as f64 / 255.0 * (BRIGHTNESS_SCALE.len() - 1) as f64) as usize;
                let character = BRIGHTNESS_SCALE.chars().nth(char_index).unwrap();
                ascii.push(character);

                let x = x * self.font_size;
                let y = y * self.font_size;

                draw_text_mut(
                    &mut img_ascii,
                    Rgba([0u8, 255u8, 0u8, 255u8]),
                    x as i32,
                    y as i32,
                    scale,
                    &font,
                    &ascii,
                );
            }
        }

        img_ascii.save(self.dest_image_dir)?;

        Ok(())
    }

    // Todo : Need rework!
    pub fn convert_to_str(&self) -> Result<String, ImageError> {
        let mut image = image::open(&self.src_image_dir)?;

        image = image.grayscale();

        let (width, height) = image.dimensions();

        let mut buf = String::new();
        for y in 0..height {
            for x in 0..width {
                if x as i32 % self.scale == 0 && y as i32 % self.scale == 0 {
                    let pixel = image.get_pixel(x, y).0[0];
                    let char_index =
                        (pixel as f64 / 255.0 * (BRIGHTNESS_SCALE.len() - 1) as f64) as usize;
                    let character = BRIGHTNESS_SCALE.chars().nth(char_index).unwrap();
                    buf.push(character);
                }
            }
            if y as i32 % self.scale == 0 {
                buf.push('\n');
            }
        }

        Ok(buf)
    }
}
