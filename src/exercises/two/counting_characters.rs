/**
 * EXERCISE 2
 * Program that accepts a string and counts how long it is
 *
 * example:
 * What is the input string? Homer
 * Homer has 5 characters
 *
 * Additional
 *  - if empty string prompt and allow them to re-enter
 *  - output contains original string
 *  - output is single string
 */
pub mod counting_characters {
  use std::io;

  pub fn main() {
    println!("Counting Characters");
    println!("What is the input string?");

    let mut user_input = String::new();

    io::stdin()
      .read_line(&mut user_input)
      .expect("Unable to read input");

    let user_input = user_input.trim();
    let input_length = user_input.chars().count();

    println!("{} has {} characters.", user_input, input_length);
  }

}
