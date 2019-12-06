/**
 * Exercise 14
 * Bill Splitter
 * Enter gross item cost: $10
 * Enter Tax rate: 10%
 * Enter service Charge 10%
 *
 * You owe a total of $12
 *
 *  added_rate = 1 + (Tax + Service)/100
 *  total = cost * added_rate
 *
 *  extra
 *  - have some preset functions added such as default Singapore rate that can be read from a file
 *
 *
 * create a tax calculator that allosws a value to be evaluted
 * for tax from a list
 * Enter the total
 * Enter the country for tax
 * display subtotal, tax and total
 *
 * example
 *
 */

pub mod bill_splitter {
  extern crate serde;
  extern crate serde_derive;
  extern crate serde_json;

  use crate::general_utils_main::general_utils::{
    get_bool_user_input, get_float_from_input, get_user_input,
  };
  use serde_derive::{Deserialize, Serialize};
  use std::fs;

  #[derive(Serialize, Deserialize, Debug)]
  struct CountryTaxes {
    sales_tax: f32,
    service_charge: f32,
  }

  #[derive(Serialize, Deserialize, Debug)]
  struct CountryPresets {
    singapore: CountryTaxes,
    uk: CountryTaxes,
  }

  pub fn main() {
    println!("Bill Splitter");
    let preset_data =
      fs::read_to_string("./presets.json").expect("Unable to read file presets.json");
    let presets: CountryPresets = serde_json::from_str(&preset_data).expect("JSON formating error");

    let user_country = get_user_input("Select a country to get sales tax from: ".to_string());
    let country = country_matcher(&user_country, presets);
    let total_cost = get_float_from_input("Amount to pay before extra charges:".to_string());
    let has_sales_tax = get_bool_user_input("Include Sales Tax? ".to_string());
    let has_service_charge = get_bool_user_input("Include Service Charge? ".to_string());

    let total_payable =
      calculate_total_cost(country, total_cost, has_sales_tax, has_service_charge);

    println!(
      "For a total cost of: {} in {} your total payable is: {:.2}",
      total_cost, user_country, total_payable
    );
  }

  fn country_matcher(country_name: &str, country_presets: CountryPresets) -> CountryTaxes {
    match country_name {
      "singapore" => country_presets.singapore,
      "uk" => country_presets.uk,
      _ => panic!("Unknown country or the developer was too lazy to implement it"),
    }
  }

  fn calculate_added_value_rate(
    country_data: CountryTaxes,
    has_sales_tax: bool,
    has_service_charge: bool,
  ) -> f32 {
    let mut taxed_percentage: f32 = 0.0;
    if has_sales_tax {
      taxed_percentage += country_data.sales_tax;
    }
    if has_service_charge {
      taxed_percentage += country_data.service_charge;
    }

    1.0 + taxed_percentage / 100.0
  }

  fn calculate_total_cost(
    country_data: CountryTaxes,
    total_cost: f32,
    has_sales_tax: bool,
    has_service_charge: bool,
  ) -> f32 {
    let added_rate = calculate_added_value_rate(country_data, has_sales_tax, has_service_charge);

    total_cost * added_rate
  }
}
