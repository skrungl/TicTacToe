pub mod game;

pub mod input {
    use std::io::{self, stdout, Write};

    #[allow(unused)]
    pub fn read_input() -> String {
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("No input recieved.");
        return s.trim().to_string();
    }

    pub fn read_input_prompt(prompt: String) -> String {
        print!("{}", prompt);
        stdout().flush().expect("failed to flush stdout");
        read_input()
    }
    
    pub fn clear_screen() {
        print!("\x1B[2J\x1B[1;1H");
    }
}