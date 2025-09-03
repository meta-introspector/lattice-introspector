// src/config.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ImageConfig {
    pub grid_width: usize,
    pub grid_height: usize,
    pub center_x: f32,
    pub center_y: f32,
    pub char_width: usize,
    pub char_height: usize,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SlimeConfig {
    pub diffusion_rate: f32,
    pub attraction_rate: f32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VisualizationConfig {
    pub screen_width: u32,
    pub screen_height: u32,
    pub debug_output_dir: String,
    pub ffmpeg_fps: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AppConfig {
    pub image_config: ImageConfig,
    pub slime_config: SlimeConfig,
    pub visualization_config: VisualizationConfig,
    pub zos_primes: Vec<u32>,
    pub a: f32, // For feedback_sequence
    pub s: f32, // For feedback_sequence
    pub run_for_prime_size: Option<u32>, // Added
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            image_config: ImageConfig {
                grid_width: 576,
                grid_height: 1024,
                center_x: 288.0,
                center_y: 512.0,
                char_width: 8,
                char_height: 16,
            },
            slime_config: SlimeConfig {
                diffusion_rate: 0.1,
                attraction_rate: 0.05,
            },
            visualization_config: VisualizationConfig {
                screen_width: 576,
                screen_height: 1024,
                debug_output_dir: "debug_frames".to_string(),
                ffmpeg_fps: 10,
            },
            zos_primes: vec![0, 1, 2, 3, 5, 7, 11, 13, 17, 19, 23],
            a: 0.8,
            s: 0.9,
            run_for_prime_size: None, // Default to None
        }
    }
}
