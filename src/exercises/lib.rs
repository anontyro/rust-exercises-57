extern crate rand;
extern crate regex;

#[path = "./one/hello_world.rs"]
pub mod hello_main;

#[path = "./two/counting_characters.rs"]
pub mod counting_char_main;

#[path = "./three/printing_quotes.rs"]
pub mod printing_quotes_main;

#[path = "./four/mad_libs.rs"]
pub mod mad_libs_main;

#[path = "./five/simple_calculator.rs"]
pub mod simple_calculator_main;

#[path = "../utils/general_utils.rs"]
pub mod general_utils_main;

#[path = "./six/retirement_calculator.rs"]
pub mod retirement_calculator_main;

#[path = "./seven/area_rectangular_room.rs"]
pub mod area_rectangular_room_main;
