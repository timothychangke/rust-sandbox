// Brings the Rng trait into scope from the external `rand` crate.
// The Rng trait provides methods for generating random numbers.
use rand::Rng;

// Imports the Ordering enum from Rust's standard library.
// This enum has three variants: Less, Greater, and Equal — used to compare values.
use std::cmp::Ordering;

// Imports the I/O (input/output) module from Rust’s standard library.
// Necessary for reading user input from the terminal.
use std::io;

// The main entry point of the program. Execution starts here.
fn main() {
    // Start an infinite loop so the user can keep guessing until they guess correctly.
    loop {
        // Display a prompt asking the user to guess a number. This is a macro
        println!("Guess a number!");

        // Generate a random number between 1 and 100 (inclusive).
        // `thread_rng()` gives us a random number generator tied to the current thread.
        // `gen_range(1..=100)` generates a number in the range 1 to 100, inclusive.
        let secret_number = rand::thread_rng().gen_range(1..=100);

        // Uncomment this for debugging: it reveals the secret number.
        // println!("The secret number is {secret_number}");

        // Ask the user to enter their guess.
        println!("Please input your guess");

        // Create a mutable variable `guess` to store the user's input.
        // It is initially an empty `String`.
        let mut guess = String::new();

        // Read a line of input from standard input (the keyboard).
        // The `read_line` function writes the input into the `guess` variable.
        // It returns a `Result`, so we use `expect` to crash with an error message if reading fails.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Convert the input string to an unsigned 32-bit integer (u32).
        // `.trim()` removes any whitespace (like newlines).
        // `.parse()` attempts the conversion, returning a Result.
        // If parsing succeeds, we use the number. If it fails, we continue the loop and prompt again.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Display the value that the user guessed.
        println!("You have guessed: {}", guess);

        // Compare the guess to the secret number using `.cmp()`, which returns an Ordering.
        // Use a `match` expression to handle the result of the comparison.
        match guess.cmp(&secret_number) {
            // If the guess is less than the secret number, tell the user it's too small.
            Ordering::Less => println!("Too small!"),

            // If the guess is greater than the secret number, tell the user it's too big.
            Ordering::Greater => println!("Too big!"),

            // If the guess is equal to the secret number, congratulate the user and exit the loop.
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
