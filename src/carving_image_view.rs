use image::{GenericImage, GenericImageView, Rgb, RgbImage};

pub struct CarvingImageView {
    width: u32,
    height: u32,
    pub full_image: RgbImage,
}

impl CarvingImageView {
    pub fn from_image(image: RgbImage) -> Self {
        let (width, height) = image.dimensions();

        Self { width, height, full_image: image }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub unsafe fn unsafe_get_pixel(&self, x: u32, y: u32) -> Rgb<u8> {
        self.full_image.unsafe_get_pixel(x, y)
    }

    pub unsafe fn unsafe_put_pixel(&mut self, x: u32, y: u32, pixel: Rgb<u8>) {
        self.full_image.unsafe_put_pixel(x, y, pixel);
    }

    pub fn sync_dimensions(&mut self) {
        let sub_image = image::imageops::crop(&mut self.full_image, 0, 0, self.width, self.height);
        self.full_image = sub_image.to_image();
    }
}
