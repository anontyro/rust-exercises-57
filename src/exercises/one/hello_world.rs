/**
 * EXERCISE 1
 * Take in input from the user and return a greeting
 *
 * extra work:
 *  - No variables
 *  - Input and output should be different variables ✓
 *  - create one of several different greatings ✓
 *
 */

pub mod hello_world {

  use rand::{thread_rng, Rng};
  use std::io;

  pub fn main() {
    println!("Greeting");
    println!("What is your name?");

    let mut user_name = String::new();

    io::stdin()
      .read_line(&mut user_name)
      .expect("Unable to read input");

    let user_name = user_name.trim();
    let greeting = get_greeting(user_name);

    println!("{}", greeting);
  }

  fn get_greeting(name: &str) -> String {
    let greeting = ["Hello", "Hi", "Hey"];

    let message = [
      "nice to meet you.",
      "hope you are doing well!",
      "good morning.",
      "good evening.",
    ];

    let greeting_num = get_rng(greeting.len());
    let message_num = get_rng(message.len());

    let msg_start: String = greeting[(greeting_num - 1) as usize].to_string();
    let msg_end: &str = message[(message_num - 1) as usize];

    let output = format!("{}, {}, {}", msg_start, name, msg_end);

    return output;
  }

  fn get_rng(max: usize) -> usize {
    let number = thread_rng().gen_range(1, max);
    return number;
  }

}
