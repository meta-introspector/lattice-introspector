use crate::image_utils::{Grid, Image, GRID_WIDTH, GRID_HEIGHT, discretize_image, feedback_sequence};

// Slime mold step: Diffusion + chemotaxis
pub fn slime_step(diffusion_rate: f32, attraction_rate: f32, food_grid: &Grid, prev_slime: &Grid) -> Grid {
    let mut new_slime = vec![vec![0.0; GRID_HEIGHT]; GRID_WIDTH];
    for x in 0..GRID_WIDTH {
        for y in 0..GRID_HEIGHT {
            // 4-neighbor diffusion
            let neighbors = [
                if x > 0 { (x - 1, y) } else { (x, y) },
                if x < GRID_WIDTH - 1 { (x + 1, y) } else { (x, y) },
                if y > 0 { (x, y - 1) } else { (x, y) },
                if y < GRID_HEIGHT - 1 { (x, y + 1) } else { (x, y) },
            ];
            let avg_neighbor = neighbors.iter().map(|&(nx, ny)| prev_slime[nx][ny]).sum::<f32>() / 4.0;

            // Gradient-based chemotaxis
            let food_grad_x = if x > 0 && x < GRID_WIDTH - 1 {
                (food_grid[x + 1][y] - food_grid[x - 1][y]) / 2.0
            } else {
                0.0
            };
            let food_grad_y = if y > 0 && y < GRID_HEIGHT - 1 {
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
pub fn initial_slime() -> Grid {
    let mut grid = vec![vec![0.0; GRID_HEIGHT]; GRID_WIDTH];
    grid[320][240] = 1.0;
    grid
}

// Meta slime sequence with history for GIF generation
pub fn meta_slime_sequence_with_history(a: f32, s: f32, diffusion_rate: f32, attraction_rate: f32, n: usize) -> Vec<Grid> {
    let mut slime = initial_slime();
    let mut history = vec![slime.clone()]; // Store initial state
    for i in 0..n {
        let feedback_img = feedback_sequence(a, s, i);
        let food_grid = discretize_image(&feedback_img);
        slime = slime_step(diffusion_rate, attraction_rate, &food_grid, &slime);
        history.push(slime.clone());
    }
    history
}
