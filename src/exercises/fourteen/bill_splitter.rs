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
  }
}
