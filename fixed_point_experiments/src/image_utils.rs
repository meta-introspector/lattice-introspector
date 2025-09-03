use std::cmp::min;
use image::{RgbImage, Rgb};

// Type aliases for clarity
pub type Point2D = (f32, f32);
use std::sync::Arc;
pub type Image = Arc<dyn Fn(Point2D) -> f32 + Send + Sync + 'static>;
pub type Grid = Vec<Vec<f32>>;

// Grid dimensions
pub const GRID_WIDTH: usize = 576;
pub const GRID_HEIGHT: usize = 1024;
pub const CENTER_X: f32 = 288.0;
pub const CENTER_Y: f32 = 512.0;

// Character rendering dimensions
pub const CHAR_WIDTH: usize = 8;
pub const CHAR_HEIGHT: usize = 16;

// Transformation: Scale point around center
pub fn transform(s: f32, p: Point2D) -> Point2D {
    let dx = p.0 - CENTER_X;
    let dy = p.1 - CENTER_Y;
    (CENTER_X + s * dx, CENTER_Y + s * dy)
}

// Background image: Constant intensity
pub fn background(_p: Point2D) -> f32 {
    1.0
}

pub fn feedback_sequence(a: f32, s: f32, n: usize) -> Image {
    let mut image: Image = Arc::new(move |p| background(p)); // Initial image
    for _ in 0..n {
        let prev_image_clone = image.clone(); // Clone the Arc
        image = Arc::new(move |p| a * prev_image_clone(transform(s, p)) + background(p));
    }
    image
}

// Discretize image to grid
pub fn discretize_image(img: &Image) -> Grid {
    let mut grid = vec![vec![0.0; GRID_HEIGHT]; GRID_WIDTH];
    for x in 0..GRID_WIDTH {
        for y in 0..GRID_HEIGHT {
            grid[x][y] = img((x as f32, y as f32));
        }
    }
    grid
}

// Loads an RgbImage from a given path
pub fn load_image_from_path(path: &str) -> RgbImage {
    image::open(path).expect("Failed to open image").to_rgb8()
}



// Converts a Grid (slime concentration) to an ASCII art string
pub fn grid_to_ascii(grid: &Grid) -> String {
    let ascii_chars = [' ', '.', ':', 'o', 'O', '#', '@']; // From low to high density
    let mut ascii_art = String::new();

    // Scale down grid to fit console output, assuming 1 character per 8x16 pixels
    let scaled_width = GRID_WIDTH / CHAR_WIDTH;
    let scaled_height = GRID_HEIGHT / CHAR_HEIGHT;

    for y_scaled in 0..scaled_height {
        for x_scaled in 0..scaled_width {
            // Average intensity for the block
            let mut sum_intensity = 0.0;
            let mut count = 0;
            for y_pixel in (y_scaled * CHAR_HEIGHT)..min((y_scaled * CHAR_HEIGHT) + CHAR_HEIGHT, GRID_HEIGHT) {
                for x_pixel in (x_scaled * CHAR_WIDTH)..min((x_scaled * CHAR_WIDTH) + CHAR_WIDTH, GRID_WIDTH) {
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
pub fn render_ascii_to_image(ascii_art: &str) -> RgbImage {
    let mut img = RgbImage::new(GRID_WIDTH as u32, GRID_HEIGHT as u32);
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
            let start_x = col * CHAR_WIDTH;
            let start_y = row * CHAR_HEIGHT;

            if start_x + CHAR_WIDTH <= GRID_WIDTH && start_y + CHAR_HEIGHT <= GRID_HEIGHT {
                for x_offset in 0..CHAR_WIDTH {
                    for y_offset in 0..CHAR_HEIGHT {
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
