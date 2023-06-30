use image::{ImageBuffer, RgbaImage, Rgba, RgbImage, Rgb};

pub fn create_circle_robot_sprite(red: bool, diameter: u32, robot_id: u8) {
    let offset = (diameter-1) as f32 / 2.0;
    let mut robot_sprite: RgbaImage = ImageBuffer::new(diameter, diameter);
    for x in 0..diameter {
        for y in 0..diameter {
            if f32::powi((x as f32) - offset, 2) + f32::powi((y as f32) - offset, 2) < f32::powi(diameter as f32 / 2.0, 2) {
                robot_sprite.put_pixel(x, y, if red { Rgba([255, 0, 0, 255]) } else { Rgba([0, 0, 255, 255]) });
            } else {
                robot_sprite.put_pixel(x, y, Rgba([0, 0, 0, 0]));
            }
        }
    }

    match (red, robot_id) {
        (true, 0) => robot_sprite.save(concat!(env!("CARGO_MANIFEST_DIR"), "/images/robot/red/robot_zero_sprite.png")).unwrap(),
        (true, 1) => robot_sprite.save(concat!(env!("CARGO_MANIFEST_DIR"), "/images/robot/red/robot_one_sprite.png")).unwrap(),
        (false, 0) => robot_sprite.save(concat!(env!("CARGO_MANIFEST_DIR"), "/images/robot/blue/robot_zero_sprite.png")).unwrap(),
        (false, 1) => robot_sprite.save(concat!(env!("CARGO_MANIFEST_DIR"), "/images/robot/blue/robot_one_sprite.png")).unwrap(),
        _ => println!("unknown robot id"),
    };
}

pub fn create_circle_robot_collider(red: bool, diameter: u32, robot_id: u8) {
    let offset = (diameter-1) as f32 / 2.0;
    let mut robot_sprite: RgbImage = ImageBuffer::new(diameter, diameter);
    for x in 0..diameter {
        for y in 0..diameter {
            if f32::powi((x as f32) - offset, 2) + f32::powi((y as f32) - offset, 2) >= f32::powi(diameter as f32 / 2.0, 2) {
                robot_sprite.put_pixel(x, y, Rgb([255, 255, 255]));
            }
        }
    }

    match (red, robot_id) {
        (true, 0) => robot_sprite.save(concat!(env!("CARGO_MANIFEST_DIR"), "/images/robot/red/robot_zero_collider.png")).unwrap(),
        (true, 1) => robot_sprite.save(concat!(env!("CARGO_MANIFEST_DIR"), "/images/robot/red/robot_one_collider.png")).unwrap(),
        (false, 0) => robot_sprite.save(concat!(env!("CARGO_MANIFEST_DIR"), "/images/robot/blue/robot_zero_collider.png")).unwrap(),
        (false, 1) => robot_sprite.save(concat!(env!("CARGO_MANIFEST_DIR"), "/images/robot/blue/robot_one_collider.png")).unwrap(),
        _ => println!("unknown robot id"),
    };
}