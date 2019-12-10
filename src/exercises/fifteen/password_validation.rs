/**
 * Exercise 15
 * Create a simple login app that checks password and username
 * against stored values and returns when correct
 *
 * example
 *
 * extras
 *  - look at how to hide user input from clear text
 *  - store the values to be read against
 *  - encypt the values using bcyrpt or similar
 */

pub mod password_validation {
  use crate::encryption_utils_main::encryption_utils::{
    decrypt_string_base64, encrypt_string_base64,
  };
  use crate::general_utils_main::general_utils::{get_user_input, get_user_password};
  use std::path::Path;

  pub struct UserData {
    username: String,
    password: String,
  }

  pub fn main() {
    println!("Password Validator");
    let user_values = get_user_data();
    if check_file_exists() {
      // open file and read
      // check for username to match
      // if none match return
    } else {
      // create new file
      // add user values as an object {username: username, password: password, data: {}}
    }
  }

  pub fn check_file_exists() -> bool {
    Path::new("./src/exercises/fifteen/store.json").exists()
  }

  pub fn get_user_data() -> UserData {
    let username = get_user_input(String::from("Enter username: "));
    let password = get_user_password(String::from("Enter password: "));
    let encrypted_password = encrypt_string_base64(password);

    let results = UserData {
      username,
      password: encrypted_password,
    };
    results
  }
}
