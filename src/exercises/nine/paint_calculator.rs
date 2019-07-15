/**
 * Exercise 8
 * Calculate gallons of paint needed to cover a ceiling
 * Prompt for L and W and assume one tin of paint covers
 * 350 sq feet
 *
 * example
 * You will need to purchase 2 gallons of paint to cover
 * 360 sq feet
 *
 * extras
 *  - Only allow numeric input
 *  - implement support for a round room
 *  - Implement support for L shaped room
 *  - Implement a mobile version that can be used at a store
 *
 */

pub mod paint_calculator {
  use general_utils_main::general_utils::get_number_from_input;

  pub fn main() {
    println!("PAINT CALCULATOR");
    let paint_covers: f32 = 360 as f32;
    let length: i32 = get_number_from_input("Length of the room in feet?".to_string());
    let width: i32 = get_number_from_input("Width of the room in feet?".to_string());
    let area_in_feet: i32 = length * width;
    let gallons_required: f32 = (area_in_feet as f32 / paint_covers).ceil();
    let paint_tins_required: i32 = gallons_required as i32;

    println!(
      "You will need to purchase {} gallons of paint to cover {} sq feet",
      paint_tins_required, area_in_feet
    );
  }
}
