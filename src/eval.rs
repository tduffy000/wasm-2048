use std::collections::HashSet;

use super::Grid;
use super::Direction;
use super::Direction::{Left, Down, Right, Up};

fn max_float(a: &f32, b: &f32) -> f32 {
    if a > b {
        *a 
    } else {
        *b
    }
}

fn min_float(a: &f32, b: &f32) -> f32 {
    if a < b {
        *a
    } else {
        *b
    }
}

impl Grid {

    fn get_grid_as_array(&self) -> [i32; 16] {
        let mut state = [0; 16];
        for i in 0..self.cells.len() {
            state[i] = match self.cells[i] {
                Some(tile) => tile.number,
                None => 0,
            };
        }
        state
    }

    pub fn evaluate(&self) -> f32 {

        let grid_array: [i32; 16] = self.get_grid_as_array();
        let weights: [f32; 3] = [0.6, 0.3, 0.1];

        let mut score: f32 = 0.0;
        score += weights[0] * self.monotonicity(&grid_array);
        score += weights[1] * self.smoothness(&grid_array);
        score += weights[2] * self.max_cornered(&grid_array) as i32 as f32;

        score
    }

    fn monotonicity(&self, grid: &[i32; 16]) -> f32 {
        let mut mono = [0.0; 4];

        for i in 0..4 {
            let row = grid[i*4..(i+1)*4]
                .iter()
                .filter(|x| **x > 0)
                .map(|x| (*x as f32).log2())
                .collect::<Vec<f32>>();

            for window in row.windows(2) {
                if window[0] > window[1] {
                    mono[0] += (window[1] - window[0]).abs();
                } else {
                    mono[1] += (window[0] - window[1]).abs();
                }
            }
        }

        for j in 0..4 {
            let col = grid
                .iter().enumerate()
                .filter(|(idx, _)| idx % 4 == j)
                .map(|(_, x)| *x as f32)
                .filter(|x| *x > 0.0)
                .map(|x| x.log2())
                .collect::<Vec<f32>>();

            for window in col.windows(2) {
                if window[0] > window[1] {
                    mono[2] += (window[1] - window[0]).abs();
                } else {
                    mono[3] += (window[0] - window[1]).abs();
                }
            }
        }

        1.0 - ((min_float(&mono[0], &mono[1]) + min_float(&mono[2], &mono[3])) / 19.0)
    }

    fn smoothness(&self, grid: &[i32; 16]) -> f32 {
        let mut smoothness = 0.0;
        for row in 0..4 {
            for col in 0..4 {
                let pos = row*4 + col;
                if grid[pos] != 0 {
                    let cell_value = (grid[pos] as f32).log2();
                    if row != 3 {
                        for i in row..4 {
                            let neighbor = grid[i*4+col] as f32;
                            if neighbor > 0.0 {
                                smoothness += (cell_value - neighbor.log2()).abs();
                            }
                        }
                    }
                    if col != 3 {
                        for j in col..4 {
                            let neighbor = grid[row*4+j] as f32;
                            if neighbor > 0.0 {
                                smoothness += (cell_value - neighbor.log2()).abs();
                            }
                        }
                    }
                }
            }
        }

        1.0 - (smoothness / 72.0)
    }

    fn max_cornered(&self, grid: &[i32; 16]) -> bool {
        let corner_positions = [0, 4, 8, 12].iter().collect::<HashSet<&usize>>();
        let max_value = grid.iter().max().unwrap();
        let max_positions = grid
            .iter()
            .enumerate()
            .filter(|(_, x)| *x == max_value)
            .map(|(idx, _)| idx);

        for pos in max_positions {
            if corner_positions.contains(&pos) {
                return true;
            }
        }
        false
    }
}

#[derive(Debug)]
pub struct BoardEval {
    grid: Grid,
    moves: [Direction; 4],
    max_depth: u8,
}

impl BoardEval {

    // TODO: we can borrow the grid; this evaluator doesn't live long
    pub fn new(grid: Grid, max_depth: u8) -> BoardEval {
        let moves: [Direction; 4] = [Left, Right, Down, Up];
        BoardEval { grid, moves, max_depth }
    }

    pub fn suggest_move(&self) -> Direction {

        let mut scores: [f32; 4] = [0.0, 0.0, 0.0, 0.0];

        for (idx, direction) in self.moves.iter().enumerate() {
            let mut grid_clone = self.grid.clone();
            grid_clone.move_in(*direction);
            scores[idx] = self.alphabeta(&grid_clone, 0, f32::MIN, f32::MAX, false);
        }

        let mut max_score: f32 = 0.0;
        let mut pos = 0;

        for (idx, score) in scores.iter().enumerate() {
            if score >= &max_score {
                max_score = *score;
                pos = idx;
            }
        }
        self.moves[pos]
    }

    fn alphabeta(&self, root: &Grid, depth: u8, mut alpha: f32, mut beta: f32, max_player: bool) -> f32 {
        if depth == self.max_depth {
            return root.evaluate();
        }
        if max_player {
            let mut value = f32::MIN;
            for direction in self.moves.iter() {
                let mut new_grid = root.clone();
                new_grid.move_in(*direction);
                value = max_float(&value, &self.alphabeta(&new_grid, depth+1, alpha, beta, false));
                alpha = max_float(&alpha, &value);
                if alpha >= beta {
                    break
                }
            }
            value
        } else {
            let mut value = f32::MAX;
            if let Some(empty_positions) = root.empty_positions() {
                for (idx, is_empty) in empty_positions.iter().enumerate() {
                    let mut new_grid = root.clone();
                    // TODO: consider 4's!
                    if *is_empty {
                        new_grid.add_tile(2, idx);
                        value = min_float(&value, &self.alphabeta(&new_grid, depth+1, alpha, beta, true));
                        beta = min_float(&beta, &value);
                        if beta <= alpha {
                            break
                        }
                    }
                }
            } else {
                return root.evaluate()
            }
            value
        }
    }
}


#[cfg(test)]
mod tests {

    use crate::{Grid, Tile};
    use std::convert::TryInto;

    // TODO: pull out from tests
    fn make_grid(from_numbers: [i32; 16]) -> Grid {
        Grid::new(
            from_numbers
                .iter()
                .map(|number| {
                    if *number > 0 {
                        Some(Tile::new(*number))
                    } else {
                        None
                    }
                })
                .collect::<Vec<Option<Tile>>>()
                .try_into()
                .unwrap(),
        )
    }

    #[test]
    fn calculates_monotonicity() {
        let fully_monotonic = make_grid([
            8, 32, 64, 512,
            4, 8, 16, 256,
            2, 4, 8, 32,
            0, 0, 4, 8,
        ]);

        let perfect_grid_array = fully_monotonic.get_grid_as_array();
        let perfect_mono = fully_monotonic.monotonicity(&perfect_grid_array);
        assert_eq!(perfect_mono, 1.0);
    }

    #[test]
    fn calculates_smoothness() {

    }

    #[test]
    fn selects_best_move() {

    }

}