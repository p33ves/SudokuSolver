use std::io;

fn main() {
    println!("Hello, world!");

    println!("Please enter the sudoku puzzle:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input!");

    println!("Your input was : {}", input);
}
