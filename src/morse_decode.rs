use std::collections::HashMap;

/**
 * Ref: https://www.codewars.com/kata/54b724efac3d5402db00065e
 *
 * Takes the morse code as input and return a decoded human-readable string.
 *
 * For example:
 *
 * ```
 * decode_morse(".... . -.--   .--- ..- -.. .") => "HEY JUDE"
 * ```
 */
use structopt::StructOpt;

fn load_morse_code_alphabet() -> HashMap<String, String> {
    let alphabet_content = include_str!("../data/morse_code.txt");
    let mut alphabet = HashMap::new();
    for line in alphabet_content.lines() {
        let mut parts = line.split('\t').into_iter();
        let english = parts.next().unwrap();
        let morse = parts.next().unwrap();
        alphabet.insert(morse.to_string(), english.to_string());
    }
    alphabet
}

fn decode_morse(encoded: &str, alphabet: &HashMap<String, String>) -> String {
    encoded
        .split("   ")
        .map(|word| {
            word.trim()
                .split(" ")
                .fold(String::new(), |mut acc, symbol| {
                    acc.push_str(alphabet.get(symbol).unwrap_or(&symbol.to_string()));
                    acc
                })
        })
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
        .to_string()
}

#[derive(Debug, StructOpt)]
pub struct Opts {
    #[structopt(
        required = true,
        help = "Morse message to decode. E.g. '.... . -.--   .---'"
    )]
    morse_message: String,
}

pub fn main(opts: Opts) {
    let alphabet = load_morse_code_alphabet();
    println!("Input: '{}'", opts.morse_message);
    let result = decode_morse(&opts.morse_message, &alphabet);
    println!("Output: '{}'", result);
}

#[cfg(test)]
mod tests {
    use super::decode_morse;
    use super::load_morse_code_alphabet;

    #[test]
    fn hey_jude() {
        let alphabet = load_morse_code_alphabet();
        assert_eq!(
            decode_morse(".... . -.--   .--- ..- -.. .", &alphabet),
            "HEY JUDE"
        );
    }

    #[test]
    fn empty() {
        let alphabet = load_morse_code_alphabet();
        assert_eq!(decode_morse("", &alphabet), "");
    }

    #[test]
    fn a_letter() {
        let alphabet = load_morse_code_alphabet();
        assert_eq!(decode_morse(".-", &alphabet), "A");
    }

    #[test]
    fn spaces() {
        let alphabet = load_morse_code_alphabet();
        assert_eq!(decode_morse("   ", &alphabet), "");
    }
}
