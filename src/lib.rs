use image::{Pixel, Rgb, Rgba};
use rand::{Rng, SeedableRng};

const WIDTH: u32 = 64;
const HEIGHT: u32 = 64;

pub struct Pattern {
    pub seed: u64,
    pub body: Vec<Rgb<u8>>,
    pub cloth: Vec<Rgba<u8>>,
}

impl Pattern {
    pub fn save(&self, path: impl AsRef<std::path::Path>) {
        let mut image_buf = image::RgbaImage::new(WIDTH, HEIGHT);
        let mut rng = rand_xoshiro::Xoshiro256PlusPlus::seed_from_u64(self.seed);

        let body_range = 0..self.body.len();
        let cloth_range = 0..self.cloth.len();

        for (x, y, pixel) in image_buf.enumerate_pixels_mut() {
            let surface_type = if y < HEIGHT / 2 {
                if y < HEIGHT / 4 && WIDTH / 2 <= x {
                    SurfaceType::Cloth
                } else {
                    SurfaceType::Body
                }
            } else {
                if HEIGHT / 4 * 3 <= y && WIDTH / 4 <= x && x < WIDTH / 4 * 3 {
                    SurfaceType::Body
                } else {
                    SurfaceType::Cloth
                }
            };

            match surface_type {
                SurfaceType::Body => {
                    let idx = rng.gen_range(body_range.clone());
                    *pixel = self.body[idx].to_rgba();
                }
                SurfaceType::Cloth => {
                    let idx = rng.gen_range(cloth_range.clone());
                    *pixel = self.cloth[idx].to_rgba();
                }
            }
        }

        image_buf.save(path).unwrap();
    }
}

enum SurfaceType {
    Body,
    Cloth,
}
