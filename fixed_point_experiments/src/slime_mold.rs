use crate::image_utils::{Grid, discretize_image, feedback_sequence};
use crate::config::ImageConfig;

// Slime mold step: Diffusion + chemotaxis
pub fn slime_step(diffusion_rate: f32, attraction_rate: f32, food_grid: &Grid, prev_slime: &Grid, config: &ImageConfig) -> Grid {
    let mut new_slime = vec![vec![0.0; config.grid_height]; config.grid_width];
    for x in 0..config.grid_width {
        for y in 0..config.grid_height {
            // 4-neighbor diffusion
            let neighbors = [
                if x > 0 { (x - 1, y) } else { (x, y) },
                if x < config.grid_width - 1 { (x + 1, y) } else { (x, y) },
                if y > 0 { (x, y - 1) } else { (x, y) },
                if y < config.grid_height - 1 { (x, y + 1) } else { (x, y) },
            ];
            let avg_neighbor = neighbors.iter().map(|&(nx, ny)| prev_slime[nx][ny]).sum::<f32>() / 4.0;

            // Gradient-based chemotaxis
            let food_grad_x = if x > 0 && x < config.grid_width - 1 {
                (food_grid[x + 1][y] - food_grid[x - 1][y]) / 2.0
            } else {
                0.0
            };
            let food_grad_y = if y > 0 && y < config.grid_height - 1 {
                (food_grid[x][y + 1] - food_grid[x][y - 1]) / 2.0
            } else {
                0.0
            };
            let move_x = attraction_rate * food_grad_x;
            let move_y = attraction_rate * food_grad_y;

            // Update concentration
            let new_conc = prev_slime[x][y]
                + diffusion_rate * (avg_neighbor - prev_slime[x][y])
                + move_x
                + move_y;
            new_slime[x][y] = new_conc.clamp(0.0, 1.0);
        }
    }
    new_slime
}

// Initial slime: 1.0 at center, 0.0 elsewhere
pub fn initial_slime(config: &ImageConfig) -> Grid {
    let mut grid = vec![vec![0.0; config.grid_height]; config.grid_width];
    grid[config.center_x as usize][config.center_y as usize] = 1.0;
    grid
}


