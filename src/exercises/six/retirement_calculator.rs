/**
 * Exercise 6
 *
 * example:
 * What is your current age? 25
 * At what age would you like to retire? 65
 * You have XX years left until you can retire
 * It' YYYY, so you can retire in YYYY
 *
 * extra:
 *  - Check for negative number and handle correctly
 */

pub mod retirement_calculator {
  use chrono::prelude::*;
  use crate::general_utils_main::general_utils::get_number_from_input;

  pub fn main() {
    println!("RETIREMENT CALCULATOR");

    let local: DateTime<Local> = Local::now();
    let year: i32 = local.year();

    let user_age: i32 = get_number_from_input("What is your current age?".to_string());
    let when_to_retire: i32 =
      get_number_from_input("At what age would you like to retire?".to_string());
    let year_until_retirement: i32 = when_to_retire - user_age;

    let output: String = get_output_string(year, year_until_retirement);
    println!("{}", output);
  }

  fn get_output_string(year: i32, year_until_retirement: i32) -> String {
    let retirement_year: i32 = year + year_until_retirement;

    if year_until_retirement < 0 {
      let retired: String = "You should already be retired! Since ".to_owned();
      let output: String = retired + &retirement_year.to_string();

      return output;
    }

    let retired: String = "You have ".to_owned();
    let years_left: &str = " years left until you can retire \nIt's ";
    let retire_at: &str = ", so you can retire in ";

    let output: String = retired
      + &year_until_retirement.to_string()
      + &years_left
      + &year.to_string()
      + &retire_at
      + &retirement_year.to_string();
    return output;
  }

}
