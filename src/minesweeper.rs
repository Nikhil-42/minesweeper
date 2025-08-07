use macroquad::rand::gen_range;

pub type Point = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameState {
    Playing,
    Win,
    Lose,
}

pub struct Minesweeper {
    game_state: GameState,
    mines: Vec<Vec<i8>>,
    flags: Vec<Point>,
    revealed: Vec<Point>,
    num_mines: usize,
}

impl Minesweeper {
    pub fn new(dimensions: (usize, usize), num_mines: usize) -> Self {
        let mut mine_positions = Vec::new();

        while mine_positions.len() < num_mines {
            let x = gen_range(0, dimensions.0);
            let y = gen_range(0, dimensions.1);
            if mine_positions.contains(&(x, y)) {
                continue; // Skip if this position already has a mine
            }
            mine_positions.push((x, y));
        }

        println!("Configuration Hash {:?}", mine_positions.iter().map(|&(x, y)| x * dimensions.0 + y).reduce(|acc, x| acc ^ x));

        let mut minesweeper = Minesweeper {
            game_state: GameState::Playing,
            mines: vec![vec![0; dimensions.0]; dimensions.1],
            flags: Vec::new(),
            revealed: Vec::new(),
            num_mines,
        };

        for (x, y) in mine_positions {
            minesweeper.mines[y as usize][x as usize] = -1; // -1 indicates a mine
            // Increment surrounding tiles
            for (nx, ny) in minesweeper.neighbors((x, y)) {
                if minesweeper.mines[ny][nx] != -1 {
                    minesweeper.mines[ny][nx] += 1; // Increment count
                }
            }
        }

        minesweeper
    }

    pub fn reveal_tile(&mut self, coords: Point) {
        if self.out_of_bounds(coords) {
            return; // Out of bounds
        }

        if self.flags.contains(&coords) {
            return; // Already revealed or flagged
        }

        // If the tile is a mine, game over
        if self.is_mine(coords) {
            self.game_state = GameState::Lose;
            return;
        }

        // Reveal the tile
        if !self.revealed.contains(&coords) {
            // Tile revealed, check for win condition
            self.revealed.push(coords);
            if self.revealed.len() == (self.mines.len() * self.mines[0].len() - self.num_mines) {
                // All non-mine tiles revealed, player wins
                self.game_state = GameState::Win;
            }
        } else {
            let flag_count = self.neighbors(coords).iter().filter(|&n| self.is_flagged(*n)).count() as i8;
            if flag_count == self.mine_count(coords) {
                // If the number of flags around the tile matches the mine count, reveal surrounding tiles
                for (nx, ny) in self.neighbors(coords) {
                    let new_coords = (nx, ny);
                    if !self.revealed.contains(&new_coords) && !self.flags.contains(&new_coords) {
                        self.reveal_tile(new_coords); // Recursive reveal
                    }
                }
            }
        }

        // If the tile is empty, reveal surrounding tiles
        if self.mine_count(coords) == 0 {
            for (nx, ny) in self.neighbors(coords) {
                let new_coords = (nx, ny);
                if !self.revealed.contains(&new_coords) && !self.flags.contains(&new_coords) {
                    self.reveal_tile(new_coords); // Recursive reveal
                }
            }
        }
    }

    pub fn toggle_flag(&mut self, coords: Point) {
        if self.revealed.contains(&coords) {
            return; // Cannot flag revealed tiles
        }
        // Toggle flag
        if !self.flags.contains(&coords) {
            self.flags.push(coords); // Flag
        } else {
            self.flags.retain(|&f| f != coords); // Unflag
        }
    }

    pub fn mine_count(&self, coords: Point) -> i8 {
        if coords.0 < self.mines[0].len() && coords.1 < self.mines.len() {
            return self.mines[coords.1][coords.0];
        } else {
            return 0; // Out of bounds
        }
    }

    pub fn total_flags(&self) -> usize {
        self.flags.len()
    }

    pub fn total_mines(&self) -> usize {
        self.num_mines
    }

    pub fn total_revealed(&self) -> usize {
        self.revealed.len()
    }

    pub fn is_mine(&self, coords: Point) -> bool {
        if self.out_of_bounds(coords) {
            return false; // Out of bounds
        }
        self.mines[coords.1][coords.0] == -1 // -1 indicates a mine
    }

    pub fn is_flagged(&self, coords: Point) -> bool {
        if self.out_of_bounds(coords) {
            return false; // Out of bounds
        }

        self.flags.contains(&coords)
    }

    pub fn is_revealed(&self, coords: Point) -> bool {
        if self.out_of_bounds(coords) {
            return false; // Out of bounds
        }

        self.revealed.contains(&coords)
    }

    pub fn current_state(&self) -> &GameState {
        &self.game_state
    }

    fn out_of_bounds(&self, coords: Point) -> bool {
        coords.0 >= self.mines[0].len() || coords.1 >= self.mines.len()
    }

    fn neighbors(&self, coords: Point) -> Vec<Point> {
        let mut neighbors = Vec::new();
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue; // Skip the tile itself
                }
                let new_x = coords.0 as i32 + dx;
                let new_y = coords.1 as i32 + dy;
                if new_x >= 0 && new_y >= 0 && (new_x as usize) < self.mines[0].len() && (new_y as usize) < self.mines.len() {
                    neighbors.push((new_x as usize, new_y as usize));
                }
            }
        }
        neighbors
    }
}