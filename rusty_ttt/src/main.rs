mod player;

use std::io::stdin;


fn what_is_your_name() -> String {
    println!("Enter your name:");
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        // "Unwraps" a result object and terminates programe
        .expect("Failed to read your name");
    your_name
}


fn main() {
    let name = what_is_your_name();
    println!(" Welcome {}", name);
}
