/**
 * Exercise 4
 * Mad libs are sinple games that require user input
 * generally inputting of a noun, verb, adjective and adverb
 * into a story
 *
 * example:
 * Enter a noun: dog
 * Enter a verb: walk
 * Enter an adjective: blue
 * Enter an adverb: quickly
 *
 * Do you walk your blue dog quickly? That's hilarious!
 *
 * Additional
 *  - Add more inputs to expand the story
 *  - Add more story selection
 *  - Implement branching story where answers determin how
 * the story is constructed
 */

pub mod mad_libs {
  use std::io;

  struct WordTypes<'a> {
    noun: &'a str,
    verb: &'a str,
    adjective: &'a str,
    adverb: &'a str,
  }

  pub fn main() {
    println!("MAD LIBS");

    let noun = get_user_input("Enter a noun".to_string());
    let verb = get_user_input("Enter a verb".to_string());
    let adjective = get_user_input("Enter an adjective".to_string());
    let adverb = get_user_input("Enter an adverb".to_string());

    let word_types = WordTypes {
      noun: &noun,
      verb: &verb,
      adjective: &adjective,
      adverb: &adverb,
    };

    let mad_lib = create_mad_lib(&word_types);
    println!("{}", mad_lib);
  }

  fn create_mad_lib(words: &WordTypes) -> String {
    let mad_lib_template = "Do you VERB your ADJECTIVE NOUN ADVERB? That's hilarious!";

    let mut output = mad_lib_template.replace("ADJECTIVE", words.adjective);
    output = output.replace("VERB", words.verb);
    output = output.replace("NOUN", words.noun);
    output = output.replace("ADVERB", words.adverb);

    return output;
  }

  fn get_user_input(prompt_msg: String) -> String {
    let mut user_input = String::new();

    println!("{}:", prompt_msg);

    io::stdin()
      .read_line(&mut user_input)
      .expect("Unable to read user selection");

    return user_input.trim().to_string();
  }

}
