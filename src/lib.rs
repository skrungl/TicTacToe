pub mod game;

#[allow(unused)]
/// Helper functions for terminal io.
pub mod input {
    use std::io::{self, stdout, Write};

    /// Returns input as String.
    pub fn read_input() -> String {
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("No input recieved.");
        return s.trim().to_string();
    }

    /// Prints text infront of text caret before awaiting input.
    /// Panics if stdout cannot be flushed.
    /// # Arguments:
    /// * 'prompt' - A String containing what you want to print.
    pub fn read_input_prompt(prompt: String) -> String {
        print!("{}", prompt);
        stdout().flush().expect("failed to flush stdout");
        read_input()
    }
    
    /// Clears the terminal.
    pub fn clear_screen() {
        print!("\x1B[2J\x1B[1;1H");
    }
}