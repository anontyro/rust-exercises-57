extern crate chrono;
extern crate rand;
extern crate regex;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

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

#[path = "./eight/pizza_party.rs"]
pub mod pizza_party_main;

#[path = "./nine/paint_calculator.rs"]
pub mod paint_calculator_main;

#[path = "./ten/self_checkout.rs"]
pub mod self_checkout_main;

#[path = "./eleven/currency_converter.rs"]
pub mod currency_converter_main;

#[path = "./twelve/compute_interest.rs"]
pub mod compute_interest_main;

#[path = "./fourteen/bill_splitter.rs"]
pub mod bill_splitter_main;

#[path = "./thirtySeven/password_generator.rs"]
pub mod password_generator_main;
