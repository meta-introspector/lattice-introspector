use image::{RgbImage, Rgb};
use solfunmeme_banner::banner_generator::generate_solfunmeme_banner;
use std::path::Path; // Added for path manipulation
use std::fs; // Added for creating directories

mod image_utils;
mod slime_mold;
mod sat_solver;
mod config; // Added
use crate::config::ImageConfig; // Added

use image_utils::{render_ascii_to_image, grid_to_ascii, feedback_sequence, discretize_image}; // Updated imports
use slime_mold::{initial_slime, slime_step}; // Updated imports
use sat_solver::{find_min_cost};


fn main() {
    // Zos primes for grid sizes and steps
    let zos_primes = vec![0, 1, 2, 3, 5, 7, 11, 13, 17, 19, 23];

    // Define a fixed screen size for visualization
    const SCREEN_WIDTH: u32 = 576;
    const SCREEN_HEIGHT: u32 = 1024;

    println!("--- Slime Concentration Calculations and Debug Visualization ---");

    for &prime_size in &zos_primes {
        // Ensure prime_size is at least 1 for grid dimensions
        let grid_dim = if prime_size == 0 { 1 } else { prime_size as usize };

        let image_config = ImageConfig {
            grid_width: grid_dim,
            grid_height: grid_dim,
            center_x: (grid_dim / 2) as f32,
            center_y: (grid_dim / 2) as f32,
            char_width: 8, // Use standard char width for ASCII art
            char_height: 16, // Use standard char height for ASCII art
        };

        let diffusion_rate = 0.1;
        let attraction_rate = 0.05;
        let iterations = grid_dim; // Use prime_size as iterations

        // Create a debug directory for this grid size
        let debug_dir = format!("debug_frames/grid_{}x{}", grid_dim, grid_dim);
        fs::create_dir_all(&debug_dir).expect("Failed to create debug directory");

        // Initial slime
        let mut slime_grid = initial_slime(&image_config);

        // Simulate slime for 'iterations' steps
        for iter in 0..iterations {
            // For this simplified calculation, we'll use a constant food grid
            let food_grid = discretize_image(&feedback_sequence(0.8, 0.9, iter, image_config.center_x, image_config.center_y), &image_config);
            slime_grid = slime_step(diffusion_rate, attraction_rate, &food_grid, &slime_grid, &image_config);

            let center_x_idx = (image_config.grid_width / 2) as usize;
            let center_y_idx = (image_config.grid_height / 2) as usize;

            let center_conc = if center_x_idx < image_config.grid_width && center_y_idx < image_config.grid_height {
                slime_grid[center_x_idx][center_y_idx]
            } else {
                0.0
            };

            println!(
                "  Grid Size: {}x{}, Iteration: {}, Center Concentration: {:.4}",
                grid_dim, grid_dim, iter, center_conc
            );

            // --- Debug Visualization ---
            let ascii_slime = grid_to_ascii(&slime_grid, &image_config);
            let mut debug_img = render_ascii_to_image(&ascii_slime, &image_config);

            // Normalize to screen size and center
            let scaled_width = debug_img.width();
            let scaled_height = debug_img.height();

            let paste_x = (SCREEN_WIDTH - scaled_width) / 2;
            let paste_y = (SCREEN_HEIGHT - scaled_height) / 2;

            let mut screen_img = RgbImage::new(SCREEN_WIDTH, SCREEN_HEIGHT);
            image::imageops::overlay(&mut screen_img, &debug_img, paste_x, paste_y);


            // Save debug PNG
            let debug_png_path = format!("{}/frame_{:05}.png", debug_dir, iter);
            screen_img.save(&debug_png_path).expect("Failed to save debug PNG");
        }

        // Generate GIF from debug PNGs
        println!("  Generating GIF for {}x{} grid...", grid_dim, grid_dim);
        let gif_output_path = format!("debug_frames/grid_{}x{}.gif", grid_dim, grid_dim);
        let ffmpeg_command = format!(
            "ffmpeg -y -i {}/frame_%05d.png -vf \"fps=10,scale={}:{}\" {}",
            debug_dir, SCREEN_WIDTH, SCREEN_HEIGHT, gif_output_path
        );
        // Note: This ffmpeg command will be executed by the shell script, not directly by Rust
        println!("    FFmpeg command: {}", ffmpeg_command);
    }

    println!("\n--- Minimal Vertex Cover Calculation ---");
    let min_cost = find_min_cost();
    println!(
        "Minimal cost (min number of true variables for C5 vertex cover): {}",
        min_cost
    );
}
