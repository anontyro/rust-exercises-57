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
  use general_utils_main::general_utils::get_number_from_input;

  pub fn main() {
    println!("AREA OF A RECTANGULAR ROOM");
    let length: i32 = get_number_from_input("What is the length of the room in feet?".to_string());
    let width: i32 = get_number_from_input("What is the width of the room in feet?".to_string());

    let area: f32 = calculate_area(length, width);
    let area_meters: f32 = calculate_area_meter(length, width);

    println!("You entered {} feet by {} feet", length, width);
    println!(
      "The area is:\n{} square feet\n{} square metres",
      area, area_meters
    )
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
