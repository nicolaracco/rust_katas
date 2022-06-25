/**
 * Ref: https://www.codewars.com/kata/5552101f47fc5178b1000050
 *
 * Some numbers have funny properties. For example:
 *   89 --> 8¹ + 9² = 89 * 1
 *   695 --> 6² + 9³ + 5⁴= 1390 = 695 * 2
 *   46288 --> 4³ + 6⁴+ 2⁵ + 8⁶ + 8⁷ = 2360688 = 46288 * 51
 *
 * Given a positive integer n written as abcd... (a, b, c, d... being digits) and a positive integer p
 *
 * - we want to find a positive integer k, if it exists, such that the sum of the digits of n taken to the successive powers of p is equal to k * n.
 *
 * In other words:
 *
 * > Is there an integer k such as : (a ^ p + b ^ (p+1) + c ^(p+2) + d ^ (p+3) + ...) = n * k
 *
 * If it is the case we will return k, if not return -1.
 * Note: n and p will always be given as strictly positive integers.Implement the function unique_in_order which takes as argument a sequence and returns a list of items without any elements with the same value next to each other and preserving the original order of elements.
 *
 * For example:
 *
 * ```
 * dig_pow(89, 1) should return 1 since 8¹ + 9² = 89 = 89 * 1
 * dig_pow(92, 1) should return -1 since there is no k such as 9¹ + 2² equals 92 * k
 * dig_pow(695, 2) should return 2 since 6² + 9³ + 5⁴= 1390 = 695 * 2
 * dig_pow(46288, 3) should return 51 since 4³ + 6⁴+ 2⁵ + 8⁶ + 8⁷ = 2360688 = 46288 * 51
 * ```
 */
use structopt::StructOpt;

fn dig_pow(n: i64, p: i32) -> i64 {
    let p: u32 = p.try_into().unwrap();
    let n_digits_sum: i64 = n
        .to_string()
        .char_indices()
        .map(|(i, c)| (c.to_digit(10).unwrap() as i64).pow(p + i as u32))
        .sum();
    match n_digits_sum % n {
        0 => n_digits_sum / n,
        _ => -1,
    }
}

#[derive(Debug, StructOpt)]
pub struct Opts {
    #[structopt(required = true, help = "a positive integer. E.g. 695")]
    n: i64,
    #[structopt(required = true, help = "a positive integer. E.g. 2")]
    p: i32,
}

pub fn main(opts: Opts) {
    println!("Input: {{ n: {}, p: {} }}", opts.n, opts.p);
    let result = dig_pow(opts.n, opts.p);
    println!("Output: {{ k: {} }}", result);
}

#[cfg(test)]
mod tests {
    use super::dig_pow;

    #[test]
    fn found() {
        assert_eq!(dig_pow(89, 1), 1);
    }

    #[test]
    fn not_found() {
        assert_eq!(dig_pow(92, 1), -1);
    }

    #[test]
    fn two() {
        assert_eq!(dig_pow(695, 2), 2);
    }

    #[test]
    fn fifty_one() {
        assert_eq!(dig_pow(46288, 3), 51);
    }

    #[test]
    fn not_found_two() {
        assert_eq!(dig_pow(46288, 5), -1);
    }
}
