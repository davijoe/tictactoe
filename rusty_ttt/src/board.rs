use tabled::Tabled;

#[derive(Tabled)]
struct Board {
    a: char,
    b: char,
    c: char,
}


fn display_board(board: &Vec<Vec<char>>) {
    for row in board {
        for cell in row {
            print!("[{}]", cell);
        }
        println!();
    }
}
