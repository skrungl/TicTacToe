use colored::Colorize;
use tictactoe::{game::Game, input::read_input_prompt};

fn main() {
    let welcome = "Tic Tac Toe (The Game) (in Rust) by Skrungl\nVersion: Whatever".magenta();
    println!("{}", welcome);
    let mut tic_tac_toe = Game { clear_screen: true };

    loop {
        let input = read_input_prompt("Press ENTER to play!\n".cyan().to_string());

        if input.is_empty() {
            break;
        } else {
            match input.as_str() {
                "tcs" | "toggle clear screen" => {
                    tic_tac_toe.clear_screen = !tic_tac_toe.clear_screen;
                    println!("{}", format!("clear screen set to: {}", tic_tac_toe.clear_screen).yellow());
                },
                _ => break
            }
        }
    }

    tic_tac_toe.run()
}