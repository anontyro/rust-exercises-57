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
  use crate::general_utils_main::general_utils::get_user_input;

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

}
