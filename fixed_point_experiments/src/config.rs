// src/config.rs

pub struct ImageConfig {
    pub grid_width: usize,
    pub grid_height: usize,
    pub center_x: f32,
    pub center_y: f32,
    pub char_width: usize,
    pub char_height: usize,
}

pub struct SlimeConfig {
    pub diffusion_rate: f32,
    pub attraction_rate: f32,
}
