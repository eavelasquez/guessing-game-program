use std::io;
use std::cmp::Ordering;
use rand::Rng;

// This function is used to generate a secret number between 1 and 100.
fn generate_secret_number() -> u32 {
    rand::thread_rng().gen_range(1..101)
}

// This function is used to get the user's guess.
fn get_user_guess() -> String {
    // Declare a variable to hold the guess.
    let mut guess = String::new();

    // Read the guess from the user.
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line.");

    guess
}
// This function is used to play the game. It will generate a random number,
// then ask the user to guess the number until they guess correctly.
fn play_game() {
    println!("Guess the number!");

    // Generate a secret number between 1 and 100.
    let secret_number = generate_secret_number();

    // Loop until the user guesses the correct number.
    loop {
        // Prompt the user to guess a number.
        println!("Please guess a number between 1 and 100:");

        let guess = get_user_guess();

        // Convert the guess to a number.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // Compare the guess to the secret number.
        match guess.cmp(&secret_number) {
            // If the guess is less to the secret number, print a message saying the guess was too low.
            Ordering::Less => println!("Your guess was too low."),
            // If the guess is greater than the secret number, print a message saying the guess was too high.
            Ordering::Greater => println!("Your guess was too high."),
            // If the guess is equal to the secret number, print a message saying the guess was correct.
            Ordering::Equal => {
                println!("You guessed the correct number!");
                break;
            }
        }
    }
}

// Driver code to test the game.
fn main() {
    play_game();
}
