use std::cmp::min;
use image::{RgbImage, Rgb};
use crate::config::ImageConfig;

// Type aliases for clarity
pub type Point2D = (f32, f32);
use std::sync::Arc;
pub type Image<'a> = Arc<dyn Fn(Point2D) -> f32 + Send + Sync + 'a>;
pub type Grid = Vec<Vec<f32>>;

// Grid dimensions


// Character rendering dimensions


// Transformation: Scale point around center
pub fn transform(s: f32, p: Point2D, config: &ImageConfig) -> Point2D {
    let dx = p.0 - config.center_x;
    let dy = p.1 - config.center_y;
    (config.center_x + s * dx, config.center_y + s * dy)
}

// Background image: Constant intensity
pub fn background(_p: Point2D) -> f32 {
    1.0
}

pub fn feedback_sequence<'a>(a: f32, s: f32, n: usize, config: &'a ImageConfig) -> Image<'a> {
    let mut image: Image = Arc::new(move |p| background(p)); // Initial image
    for _ in 0..n {
        let prev_image_clone = image.clone(); // Clone the Arc
        image = Arc::new(move |p| a * prev_image_clone(transform(s, p, config)) + background(p));
    }
    image
}

// Discretize image to grid
pub fn discretize_image(img: &Image, config: &ImageConfig) -> Grid {
    let mut grid = vec![vec![0.0; config.grid_height]; config.grid_width];
    for x in 0..config.grid_width {
        for y in 0..config.grid_height {
            grid[x][y] = img((x as f32, y as f32));
        }
    }
    grid
}

// Loads an RgbImage from a given path
pub fn load_image_from_path(path: &str) -> RgbImage {
    image::open(path).expect("Failed to open image").to_rgb8()
}

// Finds the n brightest points (x, y) in an RgbImage
pub fn find_n_brightest_points(img: &RgbImage, n: usize) -> Vec<(u32, u32)> {
    let mut points_with_intensity: Vec<((u32, u32), u32)> = Vec::new();

    for x in 0..img.width() {
        for y in 0..img.height() {
            let pixel = img.get_pixel(x, y);
            let intensity = (pixel[0] as u32 + pixel[1] as u32 + pixel[2] as u32) / 3; // Average intensity
            points_with_intensity.push(((x, y), intensity));
        }
    }

    // Sort by intensity in descending order
    points_with_intensity.sort_by(|a, b| b.1.cmp(&a.1));

    // Take the top n points
    points_with_intensity.into_iter().take(n).map(|(point, _)| point).collect()
}

// Loads a TV image from a path and resizes it
pub fn load_and_resize_tv_image(path: &str, width: u32, height: u32) -> RgbImage {
    let img = image::open(path).expect("Failed to open TV image").to_rgb8();
    image::imageops::resize(&img, width, height, image::imageops::FilterType::Lanczos3)
}

// Generates a food grid from an RgbImage based on pixel intensity
pub fn generate_food_grid_from_image(img: &RgbImage, config: &ImageConfig) -> Grid {
    let mut grid = vec![vec![0.0; config.grid_height]; config.grid_width];
    for x in 0..config.grid_width {
        for y in 0..config.grid_height {
            let pixel = img.get_pixel(x as u32, y as u32);
            let intensity = (pixel[0] as f32 + pixel[1] as f32 + pixel[2] as f32) / (3.0 * 255.0); // Normalize to 0.0-1.0
            grid[x][y] = intensity;
        }
    }
    grid
}

// Generates a food grid from an RgbImage based on pixel intensity, inverting the intensity
pub fn generate_inverted_food_grid_from_image(img: &RgbImage, config: &ImageConfig) -> Grid {
    let mut grid = vec![vec![0.0; config.grid_height]; config.grid_width];
    for x in 0..config.grid_width {
        for y in 0..config.grid_height {
            let pixel = img.get_pixel(x as u32, y as u32);
            let intensity = (pixel[0] as f32 + pixel[1] as f32 + pixel[2] as f32) / (3.0 * 255.0); // Normalize to 0.0-1.0
            grid[x][y] = 1.0 - intensity; // Invert intensity: darker pixels = higher food
        }
    }
    grid
}



// Converts a Grid (slime concentration) to an ASCII art string
pub fn grid_to_ascii(grid: &Grid, config: &ImageConfig) -> String {
    let ascii_chars = [' ', '.', ':', 'o', 'O', '#', '@']; // From low to high density
    let mut ascii_art = String::new();

    // Scale down grid to fit console output, assuming 1 character per 8x16 pixels
    let scaled_width = config.grid_width / config.char_width;
    let scaled_height = config.grid_height / config.char_height;

    for y_scaled in 0..scaled_height {
        for x_scaled in 0..scaled_width {
            // Average intensity for the block
            let mut sum_intensity = 0.0;
            let mut count = 0;
            for y_pixel in (y_scaled * config.char_height)..min((y_scaled * config.char_height) + config.char_height, config.grid_height) {
                for x_pixel in (x_scaled * config.char_width)..min((x_scaled * config.char_width) + config.char_width, config.grid_width) {
                    sum_intensity += grid[x_pixel][y_pixel];
                    count += 1;
                }
            }
            let avg_intensity = if count > 0 { sum_intensity / count as f32 } else { 0.0 };

            // Map intensity to ASCII character
            let char_index = (avg_intensity * (ascii_chars.len() - 1) as f32).round() as usize;
            ascii_art.push(ascii_chars[char_index]);
        }
        ascii_art.push('\n');
    }
    ascii_art
}

// Renders ASCII art with ANSI colors to an RgbImage
pub fn render_ascii_to_image(ascii_art: &str, config: &ImageConfig) -> RgbImage {
    let mut img = RgbImage::new(config.grid_width as u32, config.grid_height as u32);
    let mut current_color = Rgb([255, 255, 255]); // Default to white

    let mut row = 0;
    let mut col = 0;

    let mut chars = ascii_art.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\x1b' { // ANSI escape code start
            if let Some('[') = chars.peek() {
                chars.next(); // Consume '['
                let mut code_str = String::new();
                while let Some(&next_c) = chars.peek() {
                    if next_c.is_ascii_digit() || next_c == ';'
                    {
                        code_str.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                if let Some('m') = chars.peek() {
                    chars.next(); // Consume 'm'
                    // Parse ANSI code
                    let codes: Vec<u8> = code_str.split(';').filter_map(|s| s.parse().ok()).collect();
                    for code in codes {
                        match code {
                            0 => current_color = Rgb([255, 255, 255]), // Reset to white
                            30 => current_color = Rgb([0, 0, 0]),     // Black
                            31 => current_color = Rgb([255, 0, 0]),   // Red
                            32 => current_color = Rgb([0, 255, 0]),   // Green
                            33 => current_color = Rgb([255, 255, 0]), // Yellow
                            34 => current_color = Rgb([0, 0, 255]),   // Blue
                            35 => current_color = Rgb([255, 0, 255]), // Magenta
                            36 => current_color = Rgb([0, 255, 255]), // Cyan
                            37 => current_color = Rgb([255, 255, 255]),// White
                            // Add more colors as needed
                            _ => {},
                        }
                    }
                }
            }
        } else if c == '\n' { // Newline
            row += 1;
            col = 0;
        } else {
            // Draw character (simple block for now)
            let start_x = col * config.char_width;
            let start_y = row * config.char_height;

            if start_x + config.char_width <= config.grid_width && start_y + config.char_height <= config.grid_height {
                for x_offset in 0..config.char_width {
                    for y_offset in 0..config.char_height {
                        // Simple block character rendering
                        // You could replace this with a more sophisticated font rendering
                        // based on character pixels if you had font data.
                        img.put_pixel((start_x + x_offset) as u32, (start_y + y_offset) as u32, current_color);
                    }
                }
            }
            col += 1;
        }
    }
    img
}
