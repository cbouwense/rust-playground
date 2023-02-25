const ANSWER: i32 = 42;

fn main() {
    println!("Guess the answer!");

    number_guesser::run();
}

mod number_guesser {
  use std::io::stdin;
  use crate::ANSWER;

  pub fn run() {
      let guess = get_number_guess_from_user();
      display_result(guess);
  }

  fn get_number_guess_from_user() -> i32 {
      let guess_as_a_string = get_guess_from_user();
      extract_number_from_guess(guess_as_a_string)
  }

  fn get_guess_from_user() -> String {
      let mut guess = String::new();
      stdin().read_line(&mut guess).expect("Failed to read line");
      guess
  }

  fn extract_number_from_guess(guess: String) -> i32 {
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

  mod tests {
    #[cfg(test)]
    use super::*;

    #[test]
    fn test_extract_number_from_guess() {
        let guess = String::from("42");
        let guess_as_a_number = extract_number_from_guess(guess);
        assert_eq!(guess_as_a_number, 42);
    }

    #[test]
    fn test_display_result() {
        let guess = 42;
        display_result(guess);
    }
}
}
