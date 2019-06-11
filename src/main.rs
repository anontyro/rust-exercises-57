extern crate main_lib;

fn main() {
    println!("Program started");
    main_lib::hello_main::hello_world::main();
    main_lib::counting_char_main::counting_characters::main();
    main_lib::printing_quotes_main::printing_quotes::main();
    main_lib::mad_libs_main::mad_libs::main();
}
