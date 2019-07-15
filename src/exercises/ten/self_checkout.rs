/**
 * Exercise 10
 * Create a self-checkout system prompt for price and quntity
 * Calculate subtotal and use a tax rate
 * print out subtotal tax and total
 *
 * example
 * Enter the price of item: 25
 * Etner the quantity of item: 2
 *
 * extras
 *  - Allow for indeterminant amount of items to be added
 */

pub mod self_checkout {
  use general_utils_main::general_utils::{get_float_from_input, get_user_input};
  use std::collections::HashMap;

  pub fn main() {
    let mut user_basket: HashMap<String, f32> = HashMap::new();
    let mut tax_rate: f32 = 10.0;
    println!("SELF CHECKOUT");
    loop {
      println!(
        "
      Select what you would like to do:
      ---------------------------------
      (a)dd a new item
      (s)et tax rate
      (t)otal of current basket
      e(x)it
      "
      );

      let user_selection = get_user_input("user selection:".to_string()).to_lowercase();

      if user_selection == "x" {
        break;
      } else if user_selection == "a" {
        add_new_item(&mut user_basket);
      } else if user_selection == "t" {
        get_subtotal(&user_basket, tax_rate);
      } else if user_selection == "s" {
        tax_rate = set_tax_rate(&mut tax_rate);
      }
    }
  }

  fn set_tax_rate(tax: &mut f32) -> f32 {
    let new_rate = get_float_from_input("New tax rate:".to_string());
    println!(
      "New tax rate is {}%, previous tax rate is: {}%",
      new_rate, tax
    );
    return new_rate;
  }

  fn add_new_item(item_store: &mut HashMap<String, f32>) {
    let item = get_user_input("What is the item name?".to_string());
    let item_price: f32 = get_float_from_input("How much is it?".to_string());
    let item_quantity: f32 = get_float_from_input("How many will you be adding?".to_string());

    let total: f32 = item_price * item_quantity;
    item_store.insert(item.to_string(), total);
  }

  fn get_subtotal(item_store: &HashMap<String, f32>, tax_rate: f32) {
    println!("Receipt");
    println!("---------------------------------");
    let mut subtotal: f32 = 0.0;
    for (item, total) in item_store {
      println!("ITEM: {} @ ${}", item, total);
      subtotal = subtotal + total;
    }
    println!("---------------------------------");
    println!("SubTotal: ${}", subtotal);
    println!("Tax rate: {}%", tax_rate);
    let net = get_nettotal(subtotal, tax_rate);
    println!("TOTAL: ${}", net);
  }

  fn get_nettotal(subtotal: f32, tax: f32) -> f32 {
    let net = ((subtotal / 100.0) * tax) + subtotal;
    return net;
  }

}
