use std::collections::BTreeSet;

/**
 * Ref: https://www.codewars.com/kata/5656b6906de340bd1b0000ac
 *
 * Take 2 strings s1 and s2 including only letters from ato z.
 * Return a new sorted string, the longest possible, containing distinct letters - each taken only once - coming from s1 or s2.
 *
 * Examples:
 *
 * ```
 * a = "xyaabbbccccdefww"
 * b = "xxxxyyyyabklmopq"
 * longest(a, b) -> "abcdefklmopqwxy"
 *
 * a = "abcdefghijklmnopqrstuvwxyz"
 * longest(a, a) -> "abcdefghijklmnopqrstuvwxyz"
 * ```
 */
use structopt::StructOpt;

fn longest(s1: &str, s2: &str) -> String {
    s1.chars()
        .chain(s2.chars())
        .collect::<BTreeSet<_>>()
        .iter()
        .collect()
}

#[derive(Debug, StructOpt)]
pub struct Opts {
    #[structopt(required = true, help = "a string containing only letters from a to z")]
    s1: String,
    #[structopt(required = true, help = "a string containing only letters from a to z")]
    s2: String,
}

pub fn main(opts: Opts) {
    println!("Input: {{ s1: '{}', s2: '{}' }}", opts.s1, opts.s2);
    let result = longest(&opts.s1, &opts.s2);
    println!("Output: '{}'", result);
}

#[cfg(test)]
mod tests {
    use super::longest;

    #[test]
    fn one() {
        let a = "xyaabbbccccdefww";
        let b = "xxxxyyyyabklmopq";
        assert_eq!(longest(a, b), "abcdefklmopqwxy");
    }

    #[test]
    fn two() {
        let a = "abcdefghijklmnopqrstuvwxyz";
        assert_eq!(longest(a, a), "abcdefghijklmnopqrstuvwxyz");
    }
}
