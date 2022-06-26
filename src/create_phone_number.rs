/**
 * Ref: https://www.codewars.com/kata/57cebe1dc6fdc20c57000ac9
 *
 * Simple, given a string of words, return the length of the shortest word(s).
 *
 * String will never be empty and you do not need to account for different data types.
 */
use structopt::StructOpt;

fn create_phone_number(numbers: &[u8]) -> String {
    let s: String = numbers.into_iter().map(u8::to_string).collect();
    format!("({}) {}-{}", &s[..3], &s[3..6], &s[6..])
}

#[derive(Debug, StructOpt)]
pub struct Opts {
    #[structopt(required = true, help = "a list of numbers")]
    digits: Vec<u8>,
}

pub fn main(opts: Opts) {
    println!(
        "Input: {}",
        opts.digits
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    );
    let result = create_phone_number(&opts.digits);
    println!("Output: '{}'", result);
}

#[cfg(test)]
mod tests {
    use super::create_phone_number;

    #[test]
    fn one() {
        assert_eq!(
            create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]),
            "(123) 456-7890"
        );
    }

    #[test]
    fn two() {
        assert_eq!(
            create_phone_number(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
            "(111) 111-1111"
        );
    }

    #[test]
    fn three() {
        assert_eq!(
            create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 9]),
            "(123) 456-7899"
        );
    }
}
