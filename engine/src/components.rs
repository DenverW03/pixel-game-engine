pub struct Position {
    pub x: f64,
    pub y: f64,
}

pub struct Size {
    pub width: f64,
    pub height: f64,
}

#[derive(Clone, Copy)]
pub struct Velocity {
    pub x: f64,
    pub y: f64,
}

// Player component, used to identify... well, the player lol
pub struct Player {}

use image::DynamicImage;
use image::ImageReader;
use std::path::Path;

// Sprite stores the width, height of the sprite, used to flatten the input asset into the image vector.
// The Vector must be size (width * height * 4), because each pixel has RGBA associated numbers.
// 32 bits per pixel
pub struct Sprite {
    pub width: u16,
    pub height: u16,
    pub image: Option<Vec<u8>>,
}

impl Sprite {
    pub fn new(path: &str) -> Self {
        let image: DynamicImage = ImageReader::open(Path::new(path))
            .unwrap()
            .decode()
            .unwrap();

        let width = image.width();
        let height = image.height();

        let image_vec: Vec<u8> = image.to_rgba8().into_raw();

        Sprite {
            width: width as u16,
            height: height as u16,
            image: Some(image_vec),
        }
    }
}
