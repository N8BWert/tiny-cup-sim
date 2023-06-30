use image::{ImageBuffer, RgbaImage, Rgba, RgbImage, Rgb};

pub fn create_field_sprite(length: u32, width: u32, corner_radius: u32, goal_depth: u32) {
    let offset = (corner_radius * 2 - 1) as f32 / 2.0;
    let mut field_sprite: RgbaImage = ImageBuffer::new(length, width);

    // Draw top left circle
    let x_start = goal_depth;
    let x_end = goal_depth + corner_radius * 2;
    let y_start = 0;
    let y_end = corner_radius * 2;
    draw_circle(&mut field_sprite, x_start, x_end, y_start, y_end, corner_radius, offset);

    // Draw top right circle
    let x_start = length - goal_depth - corner_radius * 2;
    let x_end = length - goal_depth;
    let y_start = 0;
    let y_end = corner_radius * 2;
    draw_circle(&mut field_sprite, x_start, x_end, y_start, y_end, corner_radius, offset);

    // Draw bottom left circle
    let x_start = goal_depth;
    let x_end = goal_depth + corner_radius * 2;
    let y_start = width - corner_radius * 2;
    let y_end = width;
    draw_circle(&mut field_sprite, x_start, x_end, y_start, y_end, corner_radius, offset);

    // Draw bottom right circle
    let x_start = length - goal_depth - corner_radius * 2;
    let x_end = length - goal_depth;
    let y_start = width - corner_radius * 2;
    let y_end = width;
    draw_circle(&mut field_sprite, x_start, x_end, y_start, y_end, corner_radius, offset);

    // Draw top rectangle
    let x_start = goal_depth + corner_radius;
    let x_end = length - goal_depth - corner_radius;
    let y_start = 0;
    let y_end = corner_radius;
    draw_rectangle(&mut field_sprite, x_start, x_end, y_start, y_end);

    // Draw middle rectangle
    let x_start = goal_depth;
    let x_end = length - goal_depth;
    let y_start = corner_radius;
    let y_end = width - corner_radius;
    draw_rectangle(&mut field_sprite, x_start, x_end, y_start, y_end);

    // Draw bottom rectangle
    let x_start = goal_depth + corner_radius;
    let x_end = length - goal_depth - corner_radius;
    let y_start = width - corner_radius;
    let y_end = width;
    draw_rectangle(&mut field_sprite, x_start, x_end, y_start, y_end);

    field_sprite.save(concat!(env!("CARGO_MANIFEST_DIR"), "/images/field/field_sprite.png")).unwrap();
}

fn draw_circle(sprite: &mut RgbaImage, x_start: u32, x_end: u32, y_start: u32, y_end: u32, corner_radius: u32, offset: f32) {
    for x in x_start..x_end {
        for y in y_start..y_end {
            if f32::powi((x - x_start) as f32 - offset, 2) + f32::powi((y - y_start) as f32 - offset, 2) < f32::powi(corner_radius as f32, 2) {
                sprite.put_pixel(x, y, Rgba([0, 255, 0, 255]));
            }
        }
    }
}

fn draw_rectangle(sprite: &mut RgbaImage, x_start: u32, x_end: u32, y_start: u32, y_end: u32) {
    for x in x_start..x_end {
        for y in y_start..y_end {
            sprite.put_pixel(x, y, Rgba([0, 255, 0, 255]));
        }
    }
}

pub fn create_field_collider(length: u32, width: u32, corner_radius: u32, goal_depth: u32) {
    let offset = (corner_radius * 2 - 1) as f32 / 2.0;
    let mut field_sprite: RgbImage = ImageBuffer::new(length, width);

    // Draw top left circle
    let x_start = goal_depth;
    let x_end = goal_depth + corner_radius * 2;
    let y_start = 0;
    let y_end = corner_radius * 2;
    draw_collider_circle(&mut field_sprite, x_start, x_end, y_start, y_end, corner_radius, offset);

    // Draw top right circle
    let x_start = length - goal_depth - corner_radius * 2;
    let x_end = length - goal_depth;
    let y_start = 0;
    let y_end = corner_radius * 2;
    draw_collider_circle(&mut field_sprite, x_start, x_end, y_start, y_end, corner_radius, offset);

    // Draw bottom left circle
    let x_start = goal_depth;
    let x_end = goal_depth + corner_radius * 2;
    let y_start = width - corner_radius * 2;
    let y_end = width;
    draw_collider_circle(&mut field_sprite, x_start, x_end, y_start, y_end, corner_radius, offset);

    // Draw bottom right circle
    let x_start = length - goal_depth - corner_radius * 2;
    let x_end = length - goal_depth;
    let y_start = width - corner_radius * 2;
    let y_end = width;
    draw_collider_circle(&mut field_sprite, x_start, x_end, y_start, y_end, corner_radius, offset);

    // Draw top rectangle
    let x_start = goal_depth + corner_radius;
    let x_end = length - goal_depth - corner_radius;
    let y_start = 0;
    let y_end = corner_radius;
    draw_collider_rectangle(&mut field_sprite, x_start, x_end, y_start, y_end);

    // Draw middle rectangle
    let x_start = goal_depth;
    let x_end = length - goal_depth;
    let y_start = corner_radius;
    let y_end = width - corner_radius;
    draw_collider_rectangle(&mut field_sprite, x_start, x_end, y_start, y_end);

    // Draw bottom rectangle
    let x_start = goal_depth + corner_radius;
    let x_end = length - goal_depth - corner_radius;
    let y_start = width - corner_radius;
    let y_end = width;
    draw_collider_rectangle(&mut field_sprite, x_start, x_end, y_start, y_end);

    field_sprite.save(concat!(env!("CARGO_MANIFEST_DIR"), "/images/field/field_collider.png")).unwrap();
}

fn draw_collider_circle(sprite: &mut RgbImage, x_start: u32, x_end: u32, y_start: u32, y_end: u32, corner_radius: u32, offset: f32) {
    for x in x_start..x_end {
        for y in y_start..y_end {
            if f32::powi((x - x_start) as f32 - offset, 2) + f32::powi((y - y_start) as f32 - offset, 2) < f32::powi(corner_radius as f32, 2) {
                sprite.put_pixel(x, y, Rgb([255, 255, 255]));
            }
        }
    }
}

fn draw_collider_rectangle(sprite: &mut RgbImage, x_start: u32, x_end: u32, y_start: u32, y_end: u32) {
    for x in x_start..x_end {
        for y in y_start..y_end {
            sprite.put_pixel(x, y, Rgb([255, 255, 255]));
        }
    }
}