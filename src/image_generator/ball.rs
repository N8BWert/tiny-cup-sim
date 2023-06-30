use image::{ImageBuffer, RgbaImage, Rgba, RgbImage, Rgb};

pub fn create_ball_sprite(radius: u32) {
    let offset = (radius*2-1) as f32 / 2.0;
    let mut ball_sprite: RgbaImage = ImageBuffer::new(radius*2, radius*2);
    for x in 0..radius*2 {
        for y in 0..radius*2 {
            if f32::powi((x as f32) - offset, 2) + f32::powi((y as f32) - offset, 2) < f32::powi((radius * 2) as f32 / 2.0, 2) {
                ball_sprite.put_pixel(x, y, Rgba([255, 140, 0, 255]));
            } else {
                ball_sprite.put_pixel(x, y, Rgba([0, 0, 0, 0]));
            }
        }
    }

    ball_sprite.save(concat!(env!("CARGO_MANIFEST_DIR"), "/images/ball/ball_sprite.png")).unwrap();
}

pub fn create_ball_collider(radius: u32) {
    let offset = (radius*2-1) as f32 / 2.0;
    let mut ball_collider: RgbImage = ImageBuffer::new(radius*2, radius*2);
    for x in 0..radius*2 {
        for y in 0..radius*2 {
            if f32::powi((x as f32) - offset, 2) + f32::powi((y as f32) - offset, 2) >= f32::powi((radius*2) as f32 / 2.0, 2) {
                ball_collider.put_pixel(x, y, Rgb([255, 255, 255]));
            }
        }
    }

    ball_collider.save(concat!(env!("CARGO_MANIFEST_DIR"), "/images/ball/ball_collider.png")).unwrap();
}