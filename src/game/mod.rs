mod grid;

use std::io;

pub use grid::*;

pub struct Game {
    grid: Grid,
    current_player: Player,
}


impl Game {
    pub fn new() -> Game {
        Game {
            grid: Grid::new_default(),
            current_player: Player::RED,
        }
    }
    fn input_column_number(&self) -> usize {
        loop {
            println!("Please enter a column number:");
            let mut input = String::new();
            io::stdin().read_line(&mut input).ok();
            let input: usize = input.trim().parse().unwrap_or(0);
            if input >= 1 && input <= self.grid.get_size().columns {
                return input;
            }
        }
    }

    pub fn start(&mut self) {
        loop {
            println!("{}", self.grid);
            print!("{}'s turn, ", self.current_player);
            loop {
                let column = self.input_column_number();
                match self.grid.play(column,  self.current_player.into()) {
                    Ok(grid) => {
                        self.grid = grid;
                        break;
                    }
                    Err(e) => println!("{}", e),
                }
            }

            if let Some(winner) = self.grid.find_4_aligned() {
                println!("{}", self.grid);
                println!("{} wins! 🚀🚀🚀", winner);
                break;
            }

            self.current_player = match self.current_player {
                Player::RED => Player::YELLOW,
                Player::YELLOW => Player::RED,
            };
        }
    }
}