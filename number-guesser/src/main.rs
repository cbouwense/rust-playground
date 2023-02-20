use std::io::stdin;

const ANSWER: i32 = 42;

fn main() {
    let mut guess = String::new();

    println!("Guess the answer!");

    stdin().read_line(&mut guess).expect("Failed to read line");

    let guess_as_a_number: i32 = parse_guess(guess);
    display_result(guess_as_a_number);
}

fn parse_guess(guess: String) -> i32 {
    guess.trim().parse().expect("Please type a number!")
}

fn display_result(guess: i32) {
    println!("You guessed: {}", guess);
    display_whether_guess_is_correct_or_not(guess);
}

fn display_whether_guess_is_correct_or_not(guess: i32) {
    if guess == ANSWER {
        println!("You guessed correctly!");
    } else {
        println!("You guessed incorrectly!");
    }
}
