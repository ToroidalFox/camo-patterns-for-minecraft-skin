use camo_patterns_for_minecraft_skin::Pattern;
use image::{Rgb, Rgba};

fn main() {
    Pattern {
        seed: 0x42,
        body: vec![
            Rgb([0x88, 0x7d, 0x53]),
            Rgb([0x4d, 0x38, 0x26]),
            Rgb([0x39, 0x4a, 0x3b]),
            Rgb([0x39, 0x4a, 0x3b]),
            Rgb([0x1d, 0x1d, 0x1a]),
            Rgb([0x1c, 0x27, 0x21]),
            Rgb([0x1c, 0x27, 0x21]),
        ],
        cloth: vec![
            Rgba([0x88, 0x7d, 0x53, 0xff]),
            Rgba([0x4d, 0x38, 0x26, 0xff]),
            Rgba([0x39, 0x4a, 0x3b, 0xff]),
            Rgba([0x39, 0x4a, 0x3b, 0xff]),
            Rgba([0x1d, 0x1d, 0x1a, 0xff]),
            Rgba([0x1c, 0x27, 0x21, 0xff]),
            Rgba([0x1c, 0x27, 0x21, 0xff]),
            Rgba([0x00, 0x00, 0x00, 0x00]),
            Rgba([0x00, 0x00, 0x00, 0x00]),
            Rgba([0x00, 0x00, 0x00, 0x00]),
            Rgba([0x00, 0x00, 0x00, 0x00]),
            Rgba([0x00, 0x00, 0x00, 0x00]),
            Rgba([0x00, 0x00, 0x00, 0x00]),
            Rgba([0x00, 0x00, 0x00, 0x00]),
            Rgba([0x00, 0x00, 0x00, 0x00]),
        ],
    }
    .save("patterns/forest.png");
}
