use main_lib;

use crate::main_lib::general_utils_main::general_utils::get_number_from_input;
use std::process::exit;

fn main() {
    println!("Program started");

    loop {
        let user_number = user_program_selector();
        select_program_to_run(user_number);
    }
}

fn user_program_selector() -> i32 {
    println!(
        "
    Select which program you would like to run:
    -------------------------------------------
    ( 0 ) EXIT
    ( 1 ) Hello world
    ( 2 ) Counting characters
    ( 3 ) Printing quotes
    ( 4 ) Mad libs
    ( 5 ) Simple Calculator
    ( 6 ) Retirement Calculator
    ( 7 ) Area Rectangular Room
    ( 8 ) Pizza Party
    ( 9 ) Paint Calculator
    ( 10 ) Self Checkout
    ( 11 ) Currency Converter
    ( 12 ) Compute Interest
    ( 13 ) Bill Splitter
    ( 14 ) Password Validator
    ( 15 ) Password Generator
    "
    );
    let user_number = get_number_from_input(String::from("Your Selection:"));
    return user_number;
}

fn select_program_to_run(user_number: i32) {
    match user_number {
        0 => exit(1),
        1 => main_lib::hello_main::hello_world::main(),
        2 => main_lib::counting_char_main::counting_characters::main(),
        3 => main_lib::printing_quotes_main::printing_quotes::main(),
        4 => main_lib::mad_libs_main::mad_libs::main(),
        5 => main_lib::simple_calculator_main::simple_calculator::main(),
        6 => main_lib::retirement_calculator_main::retirement_calculator::main(),
        7 => main_lib::area_rectangular_room_main::area_rectangular_room::main(),
        8 => main_lib::pizza_party_main::pizza_party::main(),
        9 => main_lib::paint_calculator_main::paint_calculator::main(),
        10 => main_lib::self_checkout_main::self_checkout::main(),
        11 => main_lib::currency_converter_main::currency_converter::main(),
        12 => main_lib::compute_interest_main::compute_interest::main(),
        13 => main_lib::bill_splitter_main::bill_splitter::main(),
        14 => main_lib::password_validation_main::password_validation::main(),
        15 => main_lib::password_generator_main::password_generator::main(),
        _ => panic!("Unable to select the correct program"),
    }
}
