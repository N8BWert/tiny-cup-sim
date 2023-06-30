use image::{ImageBuffer, RgbaImage, Rgba};

pub fn create_goal_sprites(length: u32, depth: u32) {
    let mut goal_sprite: RgbaImage = ImageBuffer::new(depth, length);
    for x in 0..depth {
        for y in 0..length {
            goal_sprite.put_pixel(x, y, Rgba([255, 0, 0, 255]));
        }
    }

    goal_sprite.save(concat!(env!("CARGO_MANIFEST_DIR"), "/images/goal/red_goal_sprite.png")).unwrap();

    let mut goal_sprite: RgbaImage = ImageBuffer::new(depth, length);
    for x in 0..depth {
        for y in 0..length {
            goal_sprite.put_pixel(x, y, Rgba([0, 0, 255, 255]));
        }
    }

    goal_sprite.save(concat!(env!("CARGO_MANIFEST_DIR"), "/images/goal/blue_goal_sprite.png")).unwrap();
}