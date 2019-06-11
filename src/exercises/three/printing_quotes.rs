/**
 * EXERCISE 3
 * Program that takes in a quote and person and adds it to
 * an array of quotes to be viewed back
 *
 * example:
 * What is the quote? Those aren't the droids
 * Who said it? Obi Wan
 * "Those aren't the droids" - Obi Wan
 *
 * Additional
 *  - Add values to an array as object that can be recalled
 *
 * */

pub mod printing_quotes {
  use std::collections::HashMap;
  use std::io;

  pub fn main() {
    let mut famous_quotes = HashMap::new();
    println!("PRINTING QUOTES");
    loop {
      println!(
        "
      Select what you would like to do:
      ---------------------------------
      ( a )dd a new quote
      ( p )rint all the quotes
      e( x ) it
      "
      );

      let mut user_selection = String::new();

      io::stdin()
        .read_line(&mut user_selection)
        .expect("Unable to read user selection");

      let user_selection = user_selection.trim().to_lowercase();

      if user_selection == "x" {
        break;
      } else if user_selection == "a" {
        add_new_quote(&mut famous_quotes);
      } else if user_selection == "p" {
        display_store(&famous_quotes);
      }
    }
  }

  fn add_new_quote(quote_store: &mut HashMap<String, String>) {
    println!("What is the quote?");

    let mut user_quote = String::new();
    let mut user_person = String::new();

    io::stdin()
      .read_line(&mut user_quote)
      .expect("Unable to read input for quote");

    println!("Who said it?");

    io::stdin()
      .read_line(&mut user_person)
      .expect("Unable to read input for person");

    let user_quote = user_quote.trim();
    let user_person = user_person.trim();

    quote_store.insert(user_person.to_string(), user_quote.to_string());

    println!("\"{}\" - {}", user_quote, user_person);
    println!("Successfully added to list");
  }

  fn display_store(quote_store: &HashMap<String, String>) {
    for (person, quote) in quote_store {
      println!("\"{}\" - {} ", quote, person);
    }
  }

}
