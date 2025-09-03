use image::{RgbImage, Rgb};
use gif::{Encoder, Frame, Repeat};
use std::fs::File;
use std::io::BufWriter;
use solfunmeme_banner::banner_generator::generate_solfunmeme_banner;

mod image_utils;
mod slime_mold;
mod sat_solver;

use image_utils::{GRID_WIDTH, GRID_HEIGHT, render_ascii_to_image, grid_to_ascii};
use slime_mold::{meta_slime_sequence_with_history};
use sat_solver::{find_min_cost};


fn main() {
    let a = 0.8;
    let s = 0.9;
    let diffusion_rate = 0.1;
    let attraction_rate = 0.05;
    let iterations = 64;

    // Zos primes for Figlet animation
    let zos_primes = vec![0, 1, 2, 3, 5, 7, 11, 13, 17, 19, 23];
    let mut figlet_frames: Vec<RgbImage> = Vec::new();

    for (i, prime) in zos_primes.iter().enumerate() {
        let git_hash = format!("{:x}", prime); // Use prime as a simple hash
        let banner_text = generate_solfunmeme_banner(&git_hash, i, 0.1); // Use i for iteration
        let img = render_ascii_to_image(&banner_text);
        figlet_frames.push(img);
    }

    // Evaluate slime at center
    let slime_history = meta_slime_sequence_with_history(a, s, diffusion_rate, attraction_rate, iterations);
    let final_slime = slime_history.last().expect("Slime history should not be empty");
    let center_conc = if 320 < GRID_WIDTH && 240 < GRID_HEIGHT {
        final_slime[320][240]
    } else {
        0.0
    };

    // Find minimal vertex cover
    let min_cost = find_min_cost();

    println!(
        "Slime concentration at center after {} meta iterations: {:.2}",
        iterations,
        center_conc
    );
    println!(
        "Minimal cost (min number of true variables for C5 vertex cover): {}",
        min_cost
    );

    // GIF generation
    let output_path = "slime_and_figlet_simulation.gif";
    let file = File::create(output_path).expect("Could not create GIF file");
    let mut encoder = Encoder::new(
        BufWriter::new(file),
        GRID_WIDTH as u16,
        GRID_HEIGHT as u16,
        &[], // Global palette (optional)
    ).expect("Could not create GIF encoder");

    encoder.set_repeat(Repeat::Infinite).expect("Could not set GIF repeat");

    // Add Figlet frames
    for (i, img) in figlet_frames.iter().enumerate() {
        println!("Encoding Figlet frame {}/{}:", i + 1, figlet_frames.len());
        let mut frame = Frame::from_rgba(GRID_WIDTH as u16, GRID_HEIGHT as u16, &mut img.clone().into_raw());
        frame.delay = 10; // 100ms delay
        encoder.write_frame(&frame).expect("Could not write Figlet frame");
    }

    // Add Slime frames
    for (i, grid) in slime_history.iter().enumerate() {
        println!("Encoding Slime frame {}/{}:", i + 1, slime_history.len());
        let ascii_slime = grid_to_ascii(grid);
        println!("{}\n", ascii_slime); // Print ASCII art to console
        let img = render_ascii_to_image(&ascii_slime);
        let mut frame = Frame::from_rgba(GRID_WIDTH as u16, GRID_HEIGHT as u16, &mut img.into_raw());
        frame.delay = 10; // 100ms delay
        encoder.write_frame(&frame).expect("Could not write Slime frame");
    }

    println!("GIF saved to {}", output_path);
}