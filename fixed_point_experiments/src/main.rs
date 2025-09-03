use image::{RgbImage, Rgb};
use solfunmeme_banner::banner_generator::generate_solfunmeme_banner;
use std::path::Path;
use std::fs;
use clap::Parser;
use serde_yaml;
use toml; // Added
use std::io::Read; // Added

mod image_utils;
mod slime_mold;
mod sat_solver;
mod config;
use crate::config::{AppConfig, ImageConfig, SlimeConfig, VisualizationConfig};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to a configuration file (YAML or TOML)
    #[clap(short, long, value_parser)]
    config: Option<String>,

    /// Override grid width
    #[clap(long, value_parser)]
    grid_width: Option<usize>,

    /// Override grid height
    #[clap(long, value_parser)]
    grid_height: Option<usize>,

    /// Override diffusion rate
    #[clap(long, value_parser)]
    diffusion_rate: Option<f32>,

    /// Override attraction rate
    #[clap(long, value_parser)]
    attraction_rate: Option<f32>,

    /// Override ffmpeg FPS
    #[clap(long, value_parser)]
    ffmpeg_fps: Option<u32>,

    /// Run only for a specific prime size
    #[clap(long, value_parser)]
    run_for_prime: Option<u32>,
}


fn main() {
    let args = Args::parse();

    // Load configuration
    let mut app_config = if let Some(config_path_str) = args.config {
        let config_path = Path::new(&config_path_str);
        let mut file = fs::File::open(&config_path)
            .expect(&format!("Failed to open config file: {}", config_path_str));
        let mut config_str = String::new();
        file.read_to_string(&mut config_str).expect("Failed to read config file to string");

        if config_path.extension().map_or(false, |ext| ext == "yaml" || ext == "yml") {
            serde_yaml::from_str(&config_str)
                .expect(&format!("Failed to parse YAML config file: {}", config_path_str))
        } else if config_path.extension().map_or(false, |ext| ext == "toml") {
            toml::from_str(&config_str)
                .expect(&format!("Failed to parse TOML config file: {}", config_path_str))
        } else {
            panic!("Unsupported config file extension. Use .yaml, .yml, or .toml");
        }
    } else {
        AppConfig::default()
    };

    // Apply command-line overrides
    if let Some(width) = args.grid_width {
        app_config.image_config.grid_width = width;
        app_config.image_config.center_x = (width / 2) as f32;
    }
    if let Some(height) = args.grid_height {
        app_config.image_config.grid_height = height;
        app_config.image_config.center_y = (height / 2) as f32;
    }
    if let Some(diff_rate) = args.diffusion_rate {
        app_config.slime_config.diffusion_rate = diff_rate;
    }
    if let Some(attr_rate) = args.attraction_rate {
        app_config.slime_config.attraction_rate = attr_rate;
    }
    if let Some(fps) = args.ffmpeg_fps {
        app_config.visualization_config.ffmpeg_fps = fps;
    }
    if let Some(prime) = args.run_for_prime {
        app_config.run_for_prime_size = Some(prime);
    }


    // Write final configuration to publishing directory
    let publish_dir = "/data/data/com.termux/files/home/storage/downloads/slime";
    fs::create_dir_all(&publish_dir).expect("Failed to create publish directory");
    let config_output_path = format!("{}/config.yaml", publish_dir); // Always write as YAML for consistency
    let serialized_config = serde_yaml::to_string(&app_config).expect("Failed to serialize config");
    fs::write(&config_output_path, serialized_config).expect("Failed to write config to file");
    println!("Final configuration written to {}", config_output_path);


    println!("---"Slime Concentration Calculations and Debug Visualization---");

    let primes_to_run = if let Some(target_prime) = app_config.run_for_prime_size {
        vec![target_prime]
    } else {
        app_config.zos_primes.clone()
    };


    for &prime_size in &primes_to_run {
        println!("\nProcessing for prime size: {}", prime_size);
        // Ensure prime_size is at least 1 for grid dimensions
        let grid_dim = if prime_size == 0 { 1 } else { prime_size as usize };

        let image_config = ImageConfig {
            grid_width: grid_dim,
            grid_height: grid_dim,
            center_x: (grid_dim / 2) as f32,
            center_y: (grid_dim / 2) as f32,
            char_width: app_config.image_config.char_width,
            char_height: app_config.image_config.char_height,
        };

        let diffusion_rate = app_config.slime_config.diffusion_rate;
        let attraction_rate = app_config.slime_config.attraction_rate;
        let iterations = grid_dim; // Use prime_size as iterations

        // Create a debug directory for this grid size
        let debug_dir = format!("{}/grid_{}x{}", app_config.visualization_config.debug_output_dir, grid_dim, grid_dim);
        println!("  Creating debug directory: {}", debug_dir);
        fs::create_dir_all(&debug_dir).expect("Failed to create debug directory");

        // Initial slime
        println!("  Initializing slime grid...");
        let mut slime_grid = initial_slime(&image_config);
        println!("    Initial slime grid dimensions: {}x{}", image_config.grid_width, image_config.grid_height);
        println!("    Initial slime grid center concentration: {:.4}", slime_grid[image_config.grid_width / 2][image_config.grid_height / 2]);


        // Simulate slime for 'iterations' steps
        println!("  Simulating slime for {} iterations...", iterations);
        for iter in 0..iterations {
            println!("    Iteration: {}", iter);
            // For this simplified calculation, we'll use a constant food grid
            println!("      Generating food grid...");
            let food_grid = discretize_image(&feedback_sequence(app_config.a, app_config.s, iter, &image_config), &image_config);
            println!("        Food grid dimensions: {}x{}", image_config.grid_width, image_config.grid_height);

            println!("      Performing slime step...");
            slime_grid = slime_step(diffusion_rate, attraction_rate, &food_grid, &slime_grid, &image_config);

            let center_x_idx = (image_config.grid_width / 2) as usize;
            let center_y_idx = (image_config.grid_height / 2) as usize;

            let center_conc = if center_x_idx < image_config.grid_width && center_y_idx < image_config.grid_height {
                slime_grid[center_x_idx][center_y_idx]
            } else {
                0.0
            };

            println!(
                "      Center Concentration: {:.4}",
                center_conc
            );

            // --- Debug Visualization ---
            println!("      Generating ASCII art...");
            let ascii_slime = grid_to_ascii(&slime_grid, &image_config);
            // println!("        ASCII Art:\n{}", ascii_slime); // Too verbose for log

            println!("      Rendering ASCII art to image...");
            let debug_img = render_ascii_to_image(&ascii_slime, &image_config);
            println!("        Debug image dimensions: {}x{}", debug_img.width(), debug_img.height());

            // Normalize to screen size and center
            println!("      Normalizing and centering image to screen size {}x{}...", app_config.visualization_config.screen_width, app_config.visualization_config.screen_height);
            let scaled_width = debug_img.width();
            let scaled_height = debug_img.height();

            let paste_x = (app_config.visualization_config.screen_width - scaled_width) / 2;
            let paste_y = (app_config.visualization_config.screen_height - scaled_height) / 2;

            let mut screen_img = RgbImage::new(app_config.visualization_config.screen_width, app_config.visualization_config.screen_height);
            image::imageops::overlay(&mut screen_img, &debug_img, paste_x as i64, paste_y as i64);


            // Save debug PNG
            let debug_png_path = format!("{}/frame_{:05}.png", debug_dir, iter);
            println!("      Saving debug PNG: {}", debug_png_path);
            screen_img.save(&debug_png_path).expect("Failed to save debug PNG");
        }

        // Generate GIF from debug PNGs
        println!("  Generating GIF for {}x{} grid...", grid_dim, grid_dim);
        let gif_output_path = format!("{}/grid_{}x{}.gif", app_config.visualization_config.debug_output_dir, grid_dim, grid_dim);
        let ffmpeg_command = format!(
            "ffmpeg -y -i {}/frame_%05d.png -start_number 0 -vf \"fps={},scale={}:{}\" {}",
            debug_dir, app_config.visualization_config.ffmpeg_fps, app_config.visualization_config.screen_width, app_config.visualization_config.screen_height, gif_output_path
        );
        // Note: This ffmpeg command will be executed by the shell script, not directly by Rust
        println!("    FFmpeg command: {}", ffmpeg_command);
    }

    println!("\n--- Minimal Vertex Cover Calculation ---
");
    let min_cost = find_min_cost();
    println!(
        "Minimal cost (min number of true variables for C5 vertex cover): {}",
        min_cost
    );
}