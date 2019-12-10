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

  use crate::general_utils_main::general_utils::get_user_password;
  // use magic_crypt::MagicCrypt;

  pub fn main() {
    println!("Password Validator");
    let password = get_user_password(String::from("Enter password:"));
    println!("password: {}", password);
  }
}
