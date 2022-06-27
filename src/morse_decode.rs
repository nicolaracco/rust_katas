/**
 * Ref:
 *  - https://www.codewars.com/kata/54b724efac3d5402db00065e
 *  - https://www.codewars.com/kata/54b72c16cd7f5154e9000457
 *
 * Takes the morse code as input and return a decoded human-readable string.
 *
 * For example:
 *
 * ```
 * decode_morse(".... . -.--   .--- ..- -.. .") => "HEY JUDE"
 * decode_morse(&decode_bits("1100110011001100000011000000111111001100111111001111110000000000000011001111110011111100111111000000110011001111110000001111110011001100000011")) => "HEY JUDE"
 * ```
 */
use std::collections::HashMap;
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

fn find_time_unit(encoded: &str) -> usize {
    let mut min_seq: usize = usize::MAX;
    let mut acc_char = '_';
    let mut acc_len = usize::MAX;
    for c in encoded.chars() {
        if acc_char == c {
            acc_len += 1;
        } else {
            min_seq = min_seq.min(acc_len);
            acc_char = c;
            acc_len = 1;
        }
    }
    min_seq.min(acc_len)
}

fn decode_bits(encoded: &str) -> String {
    let encoded = encoded.trim_matches('0');

    let time_unit = find_time_unit(encoded);

    encoded
        .chars()
        .step_by(time_unit)
        .collect::<String>()
        .replace("111", "-")
        .replace("1", ".")
        .replace("0000000", "   ")
        .replace("000", " ")
        .replace("0", "")
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
        help = "Morse message to decode. E.g. '.... . -.--   .---', or '11001100' if -b is given"
    )]
    morse_message: String,
    #[structopt(help = "Message is in binary mode", short)]
    binary_mode: bool,
}

pub fn main(opts: Opts) {
    let alphabet = load_morse_code_alphabet();
    let message = match opts.binary_mode {
        true => decode_bits(&opts.morse_message),
        false => opts.morse_message,
    };
    println!("Input: '{}'", message);
    let result = decode_morse(&message, &alphabet);
    println!("Output: '{}'", result);
}

#[cfg(test)]
mod tests {
    use super::decode_bits;
    use super::decode_morse;
    use super::load_morse_code_alphabet;

    #[test]
    fn hey_jude_bits() {
        assert_eq!(decode_bits("1100110011001100000011000000111111001100111111001111110000000000000011001111110011111100111111000000110011001111110000001111110011001100000011"), ".... . -.--   .--- ..- -.. .".to_string());
    }

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

    #[test]
    fn extra_zeros_handling_bits() {
        assert_eq!(decode_bits("01110"), ".".to_string());
    }

    #[test]
    fn extra_zeros_handling() {
        let alphabet = load_morse_code_alphabet();
        assert_eq!(
            decode_morse(&decode_bits("01110"), &alphabet),
            "E".to_string()
        );
    }

    #[test]
    fn basic_bit() {
        assert_eq!(decode_bits("1"), ".");
    }

    #[test]
    fn basic_bit_two() {
        assert_eq!(decode_bits("1110111"), "--");
    }
}
