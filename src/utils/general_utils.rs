/**
 *
 *
 */

pub mod general_utils {
  use std::io;
  use std::process::exit;

  pub fn get_user_input(prompt_msg: String) -> String {
    let mut user_input = String::new();

    println!("{}:", prompt_msg);

    io::stdin()
      .read_line(&mut user_input)
      .expect("Unable to read user selection");

    return user_input.trim().to_string();
  }

  pub fn get_number_from_input(prompt_msg: String) -> u32 {
    let mut first_number = 0;
    let first_input = get_user_input(prompt_msg);

    match first_input.parse::<u32>() {
      Ok(val) => first_number = val,
      Err(_) => exit_program(),
    }

    return first_number;
  }

  fn exit_program() {
    println!("Unable to get integer from input");
    exit(1);
  }
}
