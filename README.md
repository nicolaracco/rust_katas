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