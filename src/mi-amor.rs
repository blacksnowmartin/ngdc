use std::io::{self, Write};

fn main() {
    // Prompt the user for input
    print!("Enter a string to reverse: ");
    io::stdout().flush().expect("Failed to flush stdout");

    // Read the input from the user
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Remove any trailing newline characters from the input
    let input = input.trim();

    // Reverse the string
    let reversed: String = input.chars().rev().collect();

    // Output the reversed string
    println!("Reversed string: {}", reversed);
}
