use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Hello, world!");

    println!("Please enter the sudoku puzzle:");
    let mut raw_input = String::new();
    let digits = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];
    io::stdin()
        .read_line(&mut raw_input)
        .expect("Failed to read input!");

    let input = raw_input.trim();
    let input_length = input.chars().count();
    println!("Your input was : {}", input_length);

    match 81.cmp(&input_length) {
        Ordering::Less => println!("This seems too long!"),
        Ordering::Greater => println!("You haven't typed the full puzzle!"),
        Ordering::Equal => println!("Validating input..."),
    }
    let char_vec: Vec<char> = input.chars().collect();
    let mut error_flag: bool = false;
    for c in char_vec {
        if digits.contains(&c) {
            continue;
        } else {
            println!("{} is an invalid entry in the input!", c);
            error_flag = true;
            break;
        }
    }
    if error_flag == false {
        println!("Input characters seem valid");
    }
}
