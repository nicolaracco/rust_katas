# Rust Katas

Code Katas written in Rust, just for fun!

```
$ git clone git@github.com:nicolaracco/rust_katas.git
$ cd rust_katas
$ cargo run
```

## Katas

### Unique in order

Returns a list of items without any elements with the same value next to each other and preserving the original order of elements

Ref: https://www.codewars.com/kata/54e6533c92449cc251001667

```
$ cargo run -- unique-in-order aaaAAAAbbcdddddd
Input: {'a', 'a', 'a', 'A', 'A', 'A', 'A', 'b', 'b', 'c', 'd', 'd', 'd', 'd', 'd', 'd'}
Output: {'a', 'A', 'b', 'c', 'd'}
```

### Dig Pow

Finds a positive integer k, if it exists, such that the sum of the digits of n taken to the successive powers of p is equal to k \* n

Ref: https://www.codewars.com/kata/5552101f47fc5178b1000050

```
$ cargo run -- dig-pow 46288 3
Input: { n: 46288, p: 3 }
Output: { k: 51 }
```

### Longest

Takes 2 strings s1 and s2 including only letters from a to z. Returns a new sorted string, the longest possible, containing distinct letters - each taken only once - coming from s1 or s2.

Ref: https://www.codewars.com/kata/5656b6906de340bd1b0000ac

```
$ cargo run -- longest asdasd xyzxyz
Input: { s1: 'asdasd', s2: 'xyzxyz' }
Output: 'adsxyz'
```
