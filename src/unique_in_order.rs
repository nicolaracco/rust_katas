/**
 * Ref: https://www.codewars.com/kata/54e6533c92449cc251001667
 *
 * Implement the function unique_in_order which takes as argument a sequence and returns a list of items without any elements with the same value next to each other and preserving the original order of elements.
 *
 * For example:
 *
 * ```
 * uniqueInOrder("AAAABBBCCDAABBB") == {'A', 'B', 'C', 'D', 'A', 'B'}
 * uniqueInOrder("ABBCcAD")         == {'A', 'B', 'C', 'c', 'A', 'D'}
 * uniqueInOrder([1,2,2,3,3])       == {1,2,3}
 * ```
 */
use structopt::StructOpt;

fn unique_in_order<T>(sequence: T) -> Vec<T::Item>
where
    T: std::iter::IntoIterator,
    T::Item: std::cmp::PartialEq + std::fmt::Debug,
{
    let mut iter = sequence.into_iter().collect::<Vec<_>>();
    iter.dedup();
    iter
}

#[derive(Debug, StructOpt)]
pub struct Opts {
    #[structopt(required = true, help = "list of items. E.g. 'AABBCD'")]
    list: String,
}

pub fn main(opts: Opts) {
    let list = &opts.list.chars().collect::<Vec<_>>();
    println!(
        "Input: {{{}}}",
        list.iter()
            .map(|c| format!("'{}'", c))
            .collect::<Vec<_>>()
            .join(", ")
    );
    let result = unique_in_order(list);
    println!(
        "Output: {{{}}}",
        result
            .iter()
            .map(|c| format!("'{}'", c))
            .collect::<Vec<_>>()
            .join(", ")
    );
}

#[cfg(test)]
mod tests {
    use super::unique_in_order;

    #[test]
    fn empty_list() {
        assert_eq!(unique_in_order(vec![] as Vec<&char>), vec![] as Vec<&char>);
    }

    #[test]
    fn one_item_list() {
        assert_eq!(unique_in_order(&vec!['a']), vec![&'a']);
    }

    #[test]
    fn two_different_items_list() {
        assert_eq!(unique_in_order(&vec!['a', 'b']), vec![&'a', &'b']);
    }

    #[test]
    fn two_equal_items_list() {
        assert_eq!(unique_in_order(&vec!['a', 'a']), vec![&'a']);
    }
}
