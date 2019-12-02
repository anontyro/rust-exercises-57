/**
 * Exercise 13
 *
 *  A = P (1 + (r/n))^nt
 * P - principal amount
 * r - annual rate of interest (r/100 to get percentage)
 * t - number of years invested
 * n - number of times interest is compounded
 * A - amount at end
 *
 * example
 * What is the principal amount? 1500
 * What is the rate? 4.3
 * what is the number of years? 6
 * What is the number of times the interest is compounded per year? 4
 * $1500 invested at 4.3% for 6 years compounded 4 times per year is $1938.84
 *
 * extras
 *
 *
 */

pub mod compute_interest {

  use general_utils_main::general_utils::get_float_from_input;

  pub fn main() {
    println!("Compute Interest in USD");
    let principal_amount = get_float_from_input("Enter Principal amount: ".to_string());
    let interest_rate = get_float_from_input("At what interest rate? ".to_string());
    let number_of_years = get_float_from_input("For how many years? ".to_string());
    let number_compounded_yearly =
      get_float_from_input("How many times is this compounded per year? ".to_string());

    let total = get_compount_interest(
      principal_amount,
      interest_rate,
      number_of_years,
      number_compounded_yearly,
    );
    println!(
      "${} invested at {}% for {} years compounded {} times per year is a total of ${:.2}",
      principal_amount, interest_rate, number_of_years, number_compounded_yearly, total
    );
  }

  fn get_compount_interest(
    principal_amount: f32,
    interest_rate: f32,
    number_of_years: f32,
    number_compounded_yearly: f32,
  ) -> f32 {
    let bracketed_vals = 1.0 + ((interest_rate / 100.0) / number_compounded_yearly);
    let pow_to_raise = number_compounded_yearly * number_of_years;
    let raised_by = f32::powf(bracketed_vals, pow_to_raise);
    let final_calc = principal_amount * raised_by;
    return final_calc;
  }
}
