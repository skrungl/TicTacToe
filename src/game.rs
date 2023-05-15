use colored::*;
use rand::{distributions::Standard, prelude::Distribution, Rng};
use std::fmt::Display;

use crate::input::*;

pub struct Board {
    pub slots: [[Player; 3]; 3],
}

impl Board {
    #[allow(unused)]
    pub fn test(&self) {
        use Player::*;
        draw_board(self);
        for slot_y in 0..3 {
            for slot_x in 0..3 {
                let pos = (slot_x, slot_y);
                match self.check_winner(pos) {
                    X => println!("{:?}: {}", pos, "X Wins!".green()),
                    O => println!("{:?}: {}", pos, "O Wins!".green()),
                    Nobody => (),
                }
            }
        }
    }

    pub fn check_winner(&self, pos: (usize, usize)) -> Player {
        use Player::*;
        match self.check_diagonal(false) {
            3 => return X,
            -3 => return O,
            _ => (),
        };
        match self.check_diagonal(true) {
            3 => return X,
            -3 => return O,
            _ => (),
        };
        match self.check_vertical(pos.0) {
            3 => return X,
            -3 => return O,
            _ => (),
        };
        match self.check_horizontal(pos.1) {
            3 => return X,
            -3 => return O,
            _ => (),
        };
        Nobody
    }

    fn check_full(&self) -> bool {
        let mut count = 0;
        for slot_y in 0..3 {
            for slot_x in 0..3 {
                count = match self.slots[slot_y][slot_x] {
                    Player::X | Player::O => count + 1,
                    Player::Nobody => count,
                }
            }
        }
        count >= 9
    }

    fn check_vertical(&self, pos: usize) -> i32 {
        let mut points = 0;
        for slot in 0..3 {
            points = match self.slots[slot as usize][pos] {
                Player::X => points + 1,
                Player::O => points - 1,
                Player::Nobody => points,
            }
        }
        points
    }

    fn check_horizontal(&self, pos: usize) -> i32 {
        let mut points = 0;
        for slot in 0..3 {
            points = match self.slots[pos][slot as usize] {
                Player::X => points + 1,
                Player::O => points - 1,
                Player::Nobody => points,
            }
        }
        points
    }

    fn check_diagonal(&self, inverted: bool) -> i32 {
        let mut points: i32 = 0;
        match inverted {
            true => {
                for slot in 0..3 {
                    points = match self.slots[slot as usize][slot as usize] {
                        Player::X => points + 1,
                        Player::O => points - 1,
                        Player::Nobody => points,
                    }
                }
            }
            false => {
                for slot in 0..3 {
                    points = match self.slots[2 - slot as usize][slot as usize] {
                        Player::X => points + 1,
                        Player::O => points - 1,
                        Player::Nobody => points,
                    }
                }
            }
        }
        points
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum Player {
    X,
    O,
    Nobody,
}

impl Player {
    fn not(&self) -> Player {
        use Player::*;
        match self {
            Player::X => O,
            Player::O => X,
            Player::Nobody => Nobody,
        }
    }
}

impl Distribution<Player> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Player {
        let rand_player: i32 = rng.gen_range(1..=2);
        match rand_player {
            1 => Player::X,
            2 => Player::O,
            _ => Player::Nobody,
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Player::X => "X",
                Player::O => "O",
                Player::Nobody => " ",
            }
        )
    }
}

pub fn run() {
    use Player::*;

    let mut running = true;

    while running {
        let mut board = Board {
            slots: [
                [Nobody, Nobody, Nobody],
                [Nobody, Nobody, Nobody],
                [Nobody, Nobody, Nobody],
            ],
        };

        let player_1 = match read_input_prompt("First player choice: ".into()).as_str() {
            "X" | "x" => X,
            "O" | "o" => O,
            _ => {
                println!("{}", "Picking random starting player...".yellow());
                let mut rng = rand::thread_rng();
                rng.gen()
            }
        };

        let player_2 = player_1.not();

        println!("Player 1: {}\nPlayer 2: {}", player_1, player_2);

        let mut turn = player_1;
        let mut turn_count = 0;
        let mut last_turn_success = false;
        let mut last_valid_pos: Option<(usize, usize)> = None;

        running = loop {
            // announce which player's turn it is
            println!(
                "{}",
                match turn {
                    Nobody => "it's nobody's turn".red(),
                    X => "it's X's turn".cyan(),
                    O => "it's O's turn".cyan(),
                }
            );

            if turn == player_1 {
                println!("{}", "Type \"quit\" to quit.".yellow());
            }

            // draw the board so players can see (only on first turn)
            if turn_count == 0 {
                draw_board(&board);
            } else if !last_turn_success {
                if let Some(p) = last_valid_pos {
                    draw_board_changes(&board, p)
                } else {
                    draw_board(&board);
                }
            }
            last_turn_success = false;

            // get input
            let input = read_input_prompt(match turn {
                Nobody => ": ".into(),
                X => "X: ".into(),
                O => "O: ".into(),
            });

            // only allow quitting for player 1
            if input.to_lowercase() == "quit".to_owned() {
                if turn == player_1 {
                    break false;
                } else {
                    println!("{}", "Only player 1 can prematurely end the game!".red());
                    continue;
                }
            }

            // try parse valid position
            let pos: (usize, usize);
            match parse_board_position(input) {
                Ok(p) => pos = p,
                Err(err) => {
                    println!("{}", err.red());
                    continue; // skips turn swap
                }
            }

            board.slots[pos.1][pos.0] = match board.slots[pos.1][pos.0] {
                X => {
                    println!("{}", "Cannot occupy this space!".red());
                    continue;
                }
                O => {
                    println!("{}", "Cannot occupy this space!".red());
                    continue;
                }
                Nobody => {
                    println!("{}", format!("Placed {} at {:?}", turn.to_string(), pos).purple());
                    last_valid_pos = Some(pos);
                    turn
                },
            };

            // draw the board so players can see again
            draw_board_changes(&board, pos);

            match board.check_winner(pos) {
                X => {
                    println!("{}", "X Wins!".green());
                    break continue_prompt();
                }
                O => {
                    println!("{}", "O Wins!".green());
                    break continue_prompt();
                }
                Nobody => match board.check_full() {
                    true => {
                        println!("{} {}", "Board is full!".cyan(), "NOBODY WINS".red());
                        break continue_prompt();
                    }
                    false => (),
                },
            };

            // swaps player turn
            turn = match turn {
                X => O,
                O => X,
                Nobody => Nobody,
            };

            turn_count = turn_count + 1;
            last_turn_success = true;
        };
    }
}

fn continue_prompt() -> bool {
    match read_input_prompt("Continue? Y/N\n".green().to_string()).as_str() {
        "Y" | "y" => true,
        _ => false,
    }
}

fn parse_board_position(input_string: String) -> Result<(usize, usize), String> {
    let pos = input_string.split_once(",");

    match pos {
        Some(t) => {
            let x;
            match t.0.trim().parse::<usize>() {
                Ok(r) => x = r,
                Err(_) => return Err(format!("Invalid X position!")),
            };
            let y;
            match t.1.trim().parse::<usize>() {
                Ok(r) => y = r,
                Err(_) => return Err(format!("Invalid Y position!")),
            };

            match (x, y) {
                (0..=2, 0..=2) => {
                    return Ok((x, y));
                }
                _ => {
                    return Err("Value outside of range!".into());
                }
            }
        }
        None => return Err("Invalid input!".into()),
    };
}

fn draw_board(board: &Board) {
    println!(
        "{}|{}|{}",
        board.slots[0][0], board.slots[0][1], board.slots[0][2]
    );
    println!("=====");
    println!(
        "{}|{}|{}",
        board.slots[1][0], board.slots[1][1], board.slots[1][2]
    );
    println!("=====");
    println!(
        "{}|{}|{}",
        board.slots[2][0], board.slots[2][1], board.slots[2][2]
    );
}

fn draw_board_changes(board: &Board, pos: (usize, usize)) {
    for y in 0..3 {
        for x in 0..3 {
            print!(
                "{}",
                if (x, y) == pos {
                    board.slots[y][x].to_string().cyan()
                } else {
                    board.slots[y][x].to_string().white()
                }
            );
            match x {
                0 | 1 => {
                    print!("|");
                },
                _ => (),
            };
        }
        match y {
            0 | 1 => {
                println!("\n=====");
            },
            _ => (),
        };
    }
    println!();
}