/**
 *
 *
 */

pub mod general_utils {
  extern crate rpassword;
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

  pub fn get_user_password(prompt_msg: String) -> String {
    println!("{}:", prompt_msg);

    let password = rpassword::read_password_from_tty(None).unwrap();
    return password.trim().to_string();
  }

  pub fn get_bool_user_input(prompt_msg: String) -> bool {
    let bool_msg = "Please enter y or n".to_string();
    println!("{}:", prompt_msg);
    let user_input = get_user_input(bool_msg);

    match user_input.as_ref() {
      "y" => true,
      "yes" => true,
      "Y" => true,
      "n" => false,
      "no" => false,
      "N" => false,
      _ => panic!("Unable to find boolean from value"),
    }
  }

  pub fn get_number_from_input(prompt_msg: String) -> i32 {
    let mut first_number = 0;
    let first_input = get_user_input(prompt_msg);

    match first_input.parse::<i32>() {
      Ok(val) => first_number = val,
      Err(_) => exit_program(),
    }

    return first_number;
  }

  pub fn get_float_from_input(prompt_msg: String) -> f32 {
    let mut first_number = 0 as f32;
    let first_input = get_user_input(prompt_msg);

    match first_input.parse::<f32>() {
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
