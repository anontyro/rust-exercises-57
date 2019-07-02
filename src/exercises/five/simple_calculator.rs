/**
 * Exercise 5
 * Create a simple calculator that will take in two numbers
 * and perform the selected operation on the numbers
 *
 * Additional
 *  - Use REGEX to determin the input values
 *  - keep a running total that can be changed
 */

pub mod simple_calculator {
  use general_utils_main::general_utils::get_user_input;
  use regex::Regex;

  pub fn main() {
    println!("SIMPLE CALCULATOR");
    // let first_number = get_number_from_input("Enter the first number".to_string());
    // let second_number = get_number_from_input("Enter the Second number".to_string());

    // println!(
    //   "first number: {}, second number: {}",
    //   first_number, second_number
    // );
    let digit_regex = Regex::new(r"\d+").unwrap();
    let operators_regex = Regex::new(r"[-+/*]").unwrap();

    let user_input = get_user_input("enter the sum to calculate:".to_string());
    let all_digits = digit_regex.captures_iter(&user_input);
    let all_operators = operators_regex.captures_iter(&user_input);
    for cap in all_digits {
      println!("{}", &cap[0]);
    }
    for operator in all_operators {
      println!("{}", &operator[0]);
    }
  }

}
