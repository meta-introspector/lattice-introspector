use image::{RgbImage};
use solfunmeme_banner::banner_generator::generate_solfunmeme_banner;
use std::path::Path; // Added for path manipulation

mod image_utils;
mod slime_mold;
mod sat_solver;

use image_utils::{GRID_WIDTH, GRID_HEIGHT, render_ascii_to_image, grid_to_ascii, load_image_from_path};
use slime_mold::{meta_slime_sequence_with_history};
use sat_solver::{find_min_cost};


fn main() {
    let a = 0.8;
    let s = 0.9;
    let diffusion_rate = 0.1;
    let attraction_rate = 0.05;
    let iterations = 64; // Number of slime mold iterations

    // Zos primes for Figlet animation (still used for banner generation)
    let zos_primes = vec![0, 1, 2, 3, 5, 7, 11, 13, 17, 19, 23];

    // Slime mold history (calculated once)
    let slime_history = meta_slime_sequence_with_history(a, s, diffusion_rate, attraction_rate, iterations);

    // Find minimal vertex cover (calculated once)
    let min_cost = find_min_cost();

    println!(
        "Slime concentration at center after {} meta iterations: {:.2}",
        iterations,
        slime_history.last().expect("Slime history should not be empty")[GRID_WIDTH / 2][GRID_HEIGHT / 2]
    );
    println!(
        "Minimal cost (min number of true variables for C5 vertex cover): {}",
        min_cost
    );

    // --- Layered Frame Generation ---
    let num_video_frames = 624; // Total number of extracted video frames
    let output_frame_dir = "layered_frames";

    for i in 1..=num_video_frames {
        let frame_path = format!("extracted_media/frames/frame_{:05}.png", i);
        let background_img = load_image_from_path(&frame_path);

        // Create a new image to draw layers on
        let mut layered_frame = RgbImage::new(GRID_WIDTH as u32, GRID_HEIGHT as u32);

        // Layer 1: Background video frame
        // Copy pixels from background_img to layered_frame
        for x in 0..GRID_WIDTH {
            for y in 0..GRID_HEIGHT {
                layered_frame.put_pixel(x as u32, y as u32, *background_img.get_pixel(x as u32, y as u32));
            }
        }

        // Layer 2: Slime mold (overlayed on top of background)
        // We need to decide how to blend the slime mold. For simplicity, let's
        // just draw it as ASCII art on top of the background.
        let slime_grid_index = (i - 1) % slime_history.len(); // Loop slime history if video is longer
        let current_slime_grid = &slime_history[slime_grid_index];
        let ascii_slime = grid_to_ascii(current_slime_grid);
        // Render ASCII art to an RgbImage (this will be the slime layer)
        let slime_layer_img = render_ascii_to_image(&ascii_slime);

        // Overlay slime_layer_img onto layered_frame
        for x in 0..GRID_WIDTH {
            for y in 0..GRID_HEIGHT {
                let slime_pixel = slime_layer_img.get_pixel(x as u32, y as u32);
                // Simple blending: if slime pixel is not white (background), draw it
                if slime_pixel[0] != 255 || slime_pixel[1] != 255 || slime_pixel[2] != 255 {
                    layered_frame.put_pixel(x as u32, y as u32, *slime_pixel);
                }
            }
        }


        // Layer 3: Figlet (overlayed on top of previous layers)
        // For simplicity, let's use the Figlet from the original zos_primes loop,
        // but we'll need to decide how to position it.
        let figlet_index = (i - 1) % zos_primes.len(); // Loop figlet frames
        let git_hash = format!("{:x}", zos_primes[figlet_index]);
        let banner_text = generate_solfunmeme_banner(&git_hash, figlet_index, 0.1);
        let figlet_img = render_ascii_to_image(&banner_text);

        // Overlay figlet_img onto layered_frame
        for x in 0..GRID_WIDTH {
            for y in 0..GRID_HEIGHT {
                let figlet_pixel = figlet_img.get_pixel(x as u32, y as u32);
                // Simple blending: if figlet pixel is not white (background), draw it
                if figlet_pixel[0] != 255 || figlet_pixel[1] != 255 || figlet_pixel[2] != 255 {
                    layered_frame.put_pixel(x as u32, y as u32, *figlet_pixel);
                }
            }
        }


        // Save the layered frame
        let output_frame_path = format!("{}/layered_frame_{:05}.png", output_frame_dir, i);
        layered_frame.save(&output_frame_path).expect("Failed to save layered frame");
    }

    println!("Generated {} layered frames in {}", num_video_frames, output_frame_dir);

    // --- Recompose Video ---
    // This part will be done using ffmpeg in a separate step after Rust code finishes
}
