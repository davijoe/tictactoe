use std::io::stdin;

mod player;
pub mod board;


fn main() {
    println!("Welcome! Let's play Tic Tac Toe!");
    println!("Enter your name:");
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        // "Unwraps" a result object and terminates programe
        .expect("Failed to read your name");

    println!("Hello {} Get ready to play", your_name);

}
