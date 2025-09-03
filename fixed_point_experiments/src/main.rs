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
use crate::image_utils::{render_ascii_to_image, generate_inverted_food_grid_from_image};

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

    // --- Load ASCII art as food source ---
    let ascii_art_path = "/data/data/com.termux/files/home/storage/github/rustc/crates/introspector/data/ascii_variants/width_80_invert_frame_00001.txt";
    println!("Loading ASCII art from: {}", ascii_art_path);
    let ascii_art_content = fs::read_to_string(ascii_art_path)
        .expect(&format!("Failed to read ASCII art file: {}", ascii_art_path));

    // Determine image dimensions from ASCII art content
    // Assuming each character is char_width x char_height pixels
    let ascii_lines: Vec<&str> = ascii_art_content.lines().collect();
    let ascii_width_chars = if ascii_lines.len() > 3 { // Skip header lines
        ascii_lines[3].len() // Get width from the first actual art line
    } else {
        80 // Default if content is too short
    };
    let ascii_height_chars = if ascii_lines.len() > 3 { // Skip header lines
        ascii_lines.len() - 3 // Subtract header lines
    } else {
        24 // Default
    };

    // Create a temporary ImageConfig for rendering the ASCII art to RgbImage
    // This config's grid_width/height should match the pixel dimensions of the RgbImage
    // that render_ascii_to_image will produce.
    let temp_image_config_for_ascii_render = ImageConfig {
        grid_width: ascii_width_chars * app_config.image_config.char_width,
        grid_height: ascii_height_chars * app_config.image_config.char_height,
        center_x: (ascii_width_chars * app_config.image_config.char_width / 2) as f32,
        center_y: (ascii_height_chars * app_config.image_config.char_height / 2) as f32,
        char_width: app_config.image_config.char_width,
        char_height: app_config.image_config.char_height,
    };

    let ascii_rgb_image = render_ascii_to_image(&ascii_art_content, &temp_image_config_for_ascii_render);
    println!("ASCII art rendered to RgbImage with dimensions: {}x{}", ascii_rgb_image.width(), ascii_rgb_image.height());

    // Generate the inverted food grid from the RgbImage
    // The food grid dimensions should match the pixel dimensions of the rendered ASCII art
    let initial_food_grid = generate_inverted_food_grid_from_image(&ascii_rgb_image, &temp_image_config_for_ascii_render);
    println!("Initial food grid generated from ASCII art.");

    // Override app_config's image_config with the dimensions of the ASCII art image
    // This ensures the slime mold simulation runs on the correct grid size
    app_config.image_config.grid_width = temp_image_config_for_ascii_render.grid_width;
    app_config.image_config.grid_height = temp_image_config_for_ascii_render.grid_height;
    app_config.image_config.center_x = temp_image_config_for_ascii_render.center_x;
    app_config.image_config.center_y = temp_image_config_for_ascii_render.center_y;

    // Set a fixed number of iterations for the slime mold simulation when using ASCII art
    let slime_iterations = 200; // Example fixed iterations


    println!("---"Slime Concentration Calculations and Debug Visualization---");

    println!("\n--- Slime Concentration Calculations and Debug Visualization ---");

    // Use the dimensions from the loaded ASCII art
    let image_config = ImageConfig {
        grid_width: app_config.image_config.grid_width,
        grid_height: app_config.image_config.grid_height,
        center_x: app_config.image_config.center_x,
        center_y: app_config.image_config.center_y,
        char_width: app_config.image_config.char_width,
        char_height: app_config.image_config.char_height,
    };

    let diffusion_rate = app_config.slime_config.diffusion_rate;
    let attraction_rate = app_config.slime_config.attraction_rate;
    let iterations = slime_iterations; // Use fixed iterations

    // Create a debug directory for this run
    let debug_dir = format!("{}/ascii_slime_run", app_config.visualization_config.debug_output_dir);
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
        // Use the pre-generated food grid from ASCII art
        let food_grid = initial_food_grid.clone(); // Clone for each iteration if needed, or pass by reference

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
    println!("  Generating GIF for ASCII slime run...");
    let gif_output_path = format!("{}/ascii_slime_run.gif", app_config.visualization_config.debug_output_dir);
    let ffmpeg_command = format!(
        "ffmpeg -y -i {}/frame_%05d.png -start_number 0 -vf \"fps={},scale={}:{}\" {}",
        debug_dir,
        app_config.visualization_config.ffmpeg_fps,
        app_config.visualization_config.screen_width,
        app_config.visualization_config.screen_height,
        gif_output_path
    );
    // Note: This ffmpeg command will be executed by the shell script, not directly by Rust
    println!("    FFmpeg command: {}", ffmpeg_command);

    // Copy GIF to the publish directory
    let final_gif_publish_path = format!("{}/ascii_slime_run.gif", publish_dir);
    fs::copy(&gif_output_path, &final_gif_publish_path)
        .expect(&format!("Failed to copy GIF from {} to {}", gif_output_path, final_gif_publish_path));
    println!("  Copied GIF to publish directory: {}", final_gif_publish_path);

    println!("\n--- Minimal Vertex Cover Calculation ---
");
    let min_cost = find_min_cost();
    println!(
        "Minimal cost (min number of true variables for C5 vertex cover): {}",
        min_cost
    );
}