/**
 * Exercise 11
 * Write a program to convert one currency to another
 *
 * amount TO = (amount FROM x rate FROM) / rate TO
 *
 * example
 *
 *
 * extras
 *  - make external api call to get data
 *  - store the values to be used for converting
 */

pub mod currency_converter {
  extern crate reqwest;
  extern crate serde;
  extern crate serde_derive;
  extern crate serde_json;

  use general_utils_main::general_utils::{get_float_from_input, get_user_input};
  use serde_derive::{Deserialize, Serialize};

  #[derive(Serialize, Deserialize, Debug)]
  struct CurrencyRates {
    CAD: f64,
    HKD: f64,
    ISK: f64,
    PHP: f64,
    DKK: f64,
    HUF: f64,
    CZK: f64,
    GBP: f64,
    RON: f64,
    SEK: f64,
    IDR: f64,
    INR: f64,
    BRL: f64,
    RUB: f64,
    HRK: f64,
    JPY: f64,
    THB: f64,
    CHF: f64,
    EUR: f64,
    MYR: f64,
    BGN: f64,
    TRY: f64,
    CNY: f64,
    NOK: f64,
    NZD: f64,
    ZAR: f64,
    USD: f64,
    MXN: f64,
    SGD: f64,
    AUD: f64,
    ILS: f64,
    KRW: f64,
    PLN: f64,
  }

  #[derive(Serialize, Deserialize, Debug)]
  struct ExchangeApi {
    base: String,
    date: String,
    rates: CurrencyRates,
  }

  pub fn main() {
    println!("CURRENCY CONVERTER");
    let mut current_base = "SGD";

    let user_msg = format!(
      "{}:{} {}",
      "current base is set to", current_base, "update to any three digit ISO currency"
    );
    let user_base = get_user_input(user_msg).to_uppercase();
    let user_conversion_currency =
      get_user_input("Enter 3 digit ISO currency code to convert to".to_string()).to_uppercase();
    let user_amount = get_float_from_input("Enter amount to convert".to_string());
    current_base = &user_base;

    let rates = get_currency_data(current_base).unwrap();
    let selected_rate = get_selected_conversion_rate(&user_conversion_currency, &rates);
    let new_currency_amount = calculate_new_currency_amount(user_amount, selected_rate);

    // println!("{:?}", rates);
    println!(
      "You are converting {} {} to a new rate of {:.2} {}",
      user_amount, current_base, new_currency_amount, user_conversion_currency
    );
  }

  fn get_currency_data(base: &str) -> Result<CurrencyRates, Box<std::error::Error>> {
    let request_url = "https://api.exchangeratesapi.io/latest?base";
    let url = format!("{}={}", request_url, base);
    println!("{}", url);
    let resp: ExchangeApi = reqwest::get(&url)?.json()?;
    let rates: CurrencyRates = resp.rates;

    Ok(rates)
  }

  fn get_selected_conversion_rate(currency_iso: &str, rates: &CurrencyRates) -> f32 {
    match currency_iso {
      "USD" => rates.USD as f32,
      "SGD" => rates.SGD as f32,
      "GBP" => rates.GBP as f32,
      _ => panic!("unknown field"),
    }
  }

  fn calculate_new_currency_amount(amount_to_convert: f32, selected_rate: f32) -> f32 {
    amount_to_convert * selected_rate
  }

  // fn calculate_currency_amount(rates: CurrencyRates, currencyTo: IsoCurrencyCodes, amountToConvert: f32) {
  //   let conversionRate = rates[currencyTo];

  // }

}
