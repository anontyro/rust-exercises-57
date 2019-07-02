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
  use general_utils_main::general_utils::get_number_from_input;

  pub fn main() {
    println!("RETIREMENT CALCULATOR");

    let local: DateTime<Local> = Local::now();
    let year: i32 = local.year();

    let user_age: i32 = get_number_from_input("What is your current age?".to_string());
    let when_to_retire: i32 =
      get_number_from_input("At what age would you like to retire?".to_string());
    let year_until_retirement: i32 = when_to_retire - user_age;
    let retirement_year: i32 = year + year_until_retirement;

    if year_until_retirement < 0 {
      println!("You should already be retired since {}", retirement_year);
    } else {
      println!(
        "You have {} years left until you can retire",
        year_until_retirement
      );
      println!("It's {}, so you can retire in {}", year, retirement_year);
    }
  }

}
