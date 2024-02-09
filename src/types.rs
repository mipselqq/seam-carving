use image::{ImageBuffer, Rgb, SubImage};

pub type SubImageOfRgbBuffer<'a> = SubImage<&'a mut ImageBuffer<Rgb<u8>, Vec<u8>>>;
