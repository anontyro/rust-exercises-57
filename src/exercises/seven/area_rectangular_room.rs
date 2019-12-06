/**
 * Exercise 7
 *
 * example
 * What is the length of the room in feet? 15
 * What is the width of the room in feet? 20
 * You entered 15 feet by 20 feet
 * The area is:
 * 300 square feet
 * 27.871 square metres
 *
 * m^2 = f^2 x 0.09290304
 *
 */

pub mod area_rectangular_room {
  use crate::general_utils_main::general_utils::{get_number_from_input, get_user_input};

  pub fn main() {
    println!("AREA OF A RECTANGULAR ROOM");
    let selection = get_user_input("Convert from (f)eet or (m)etres?".to_string());
    let select_word = get_value_from_selection(&selection);
    let length: i32 = get_number_from_input(get_user_prompt("length", &select_word));
    let width: i32 = get_number_from_input(get_user_prompt("width", &select_word));

    let area: f32 = calculate_area(length, width);

    if select_word == "feet" {
      let area_meters: f32 = calculate_area_meter(length, width);

      println!("You entered {} feet by {} feet", length, width);
      println!(
        "The area is:\n{} square feet\n{} square metres",
        area, area_meters
      )
    } else {
      let area_feet: f32 = calculate_area_feet(length, width);

      println!("You entered {} metres by {} metres", length, width);
      println!(
        "The area is:\n{} square metres\n{} square feet",
        area, area_feet
      )
    }
  }

  fn get_user_prompt(first_replace: &str, select_word: &str) -> String {
    let prompt_template = "What is the XXX of the room in YYY?";
    let mut output = prompt_template.replace("XXX", first_replace);
    output = output.replace("YYY", select_word);

    return output;
  }

  fn get_value_from_selection(selection: &str) -> String {
    if selection.to_lowercase().contains("f") {
      return "feet".to_string();
    } else {
      return "metres".to_string();
    }
  }

  fn calculate_area(length: i32, width: i32) -> f32 {
    let l: f32 = length as f32;
    let w: f32 = width as f32;
    let area: f32 = l * w;

    return area;
  }

  fn calculate_area_feet(length: i32, width: i32) -> f32 {
    let area: f32 = calculate_area(length, width);
    let area_feet: f32 = area * 10.76;

    return area_feet;
  }

  fn calculate_area_meter(length: i32, width: i32) -> f32 {
    let area: f32 = calculate_area(length, width);
    let area_meter: f32 = area * 0.09290304;

    return area_meter;
  }

}
