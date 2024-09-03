use std::io::{self, Write};

fn main() {
    // Prompt the user for a number
    print!("Enter a positive integer: ");
    io::stdout().flush().expect("Failed to flush stdout");

    // Read the input from the user
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Parse the input to a u64 (unsigned 64-bit integer)
    let number: u64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid input. Please enter a valid positive integer.");
            return;
        }
    };

    // Check if the number is non-negative
    if number == 0 {
        println!("You entered 0!");
        println!("The factorial of 0 is 1, which is a special case in math.");
        println!("The Fibonacci sequence starts with 0, so the Fibonacci number of 0 is 0.");
    } else {
        // Compute and output the factorial
        let result_factorial = factorial(number);
        println!("You entered {}!", number);
        println!("The factorial of {} ({}!) is equal to {}.", number, number, result_factorial);
        println!("To put it simply, {}! means multiplying all whole numbers from {} down to 1.", number, number);

        // Compute and output the Fibonacci number
        let result_fibonacci = fibonacci(number);
        println!("The Fibonacci sequence is a series of numbers where a number is the sum of the two preceding ones, usually starting with 0 and 1.");
        println!("The Fibonacci number at position {} in the sequence is {}.", number, result_fibonacci);
    }
}

// Function to compute the factorial of a number
fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

// Function to compute the Fibonacci number of a given number
fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}