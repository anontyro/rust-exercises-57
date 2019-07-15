/**
 *  Exericse 8
 *
 * example
 * How many people? 8
 * How many pizzas do you have? 2
 *
 * 8 people with 2 pizzas
 * Each person gets 2 pieces of pizza
 * There are 0 leftover pieces
 *
 * Extras
 *  - handle correct plurilisation of the message
 *  - create a variant that prompts number of people and
 *  how many slices each want to calculate the number of pizzas
 *
 */

pub mod pizza_party {
  use general_utils_main::general_utils::get_number_from_input;

  pub fn main() {
    println!("PIZZA PARTY!!!");
    let people = get_number_from_input("How many people?".to_string());
    let pizzas = get_number_from_input("How many pizzas do you have?".to_string());
    let slices = get_number_from_input("How many slices does each pizza have?".to_string());
    let total_slices = pizzas * slices;
    let amount_per_person: f32 = total_slices as f32 / people as f32;
    let rounded_amount: u32 = amount_per_person as u32;
    let left_over: f32 = (amount_per_person - rounded_amount as f32) * people as f32;

    println!("{} people with {} pizzas", people, pizzas);
    println!("Each person gets {} pieces of pizza", rounded_amount);
    println!("There are {:.0} leftover pieces", left_over);
  }
}
