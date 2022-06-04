mod grid;
mod ai;

use std::io;

pub use grid::*;
pub use ai::*;

//const GAMEMODE: GameMode = GameMode::HumanVsHuman;
const GAMEMODE: GameMode = GameMode::HumanVsAI;

pub struct Game {
    grid: Grid,
    gamemode: GameMode,
    current_player: Player,
    ai: Option<AI>
}

enum GameMode {
    HumanVsHuman,
    HumanVsAI,
}

impl Game {
    pub fn new() -> Game {
        let ai = match GAMEMODE {
            GameMode::HumanVsHuman => None,
            GameMode::HumanVsAI => Some(AI::new(100))
        };

        Game {
            grid: Grid::new_default(),
            current_player: Player::RED,
            gamemode: GAMEMODE,
            ai
        }
    }

    fn get_ai(&self) -> &AI {
        self.ai.as_ref().unwrap()
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
                let column = match self.current_player {
                    Player::RED => self.input_column_number(),
                    Player::YELLOW => match self.gamemode {
                        GameMode::HumanVsHuman => self.input_column_number(),
                        GameMode::HumanVsAI => self.get_ai().get_column(&self.grid, self.current_player),
                    },
                };

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

            self.current_player = self.current_player.opponent();
        }
    }
}