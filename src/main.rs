use structopt::StructOpt;

mod dig_pow;
mod unique_in_order;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "rust_katas",
    about = "A collection of code katas written in Rust"
)]
enum Opts {
    #[structopt(
        about = "returns a list of items without any elements with the same value next to each other and preserving the original order of elements"
    )]
    UniqueInOrder(unique_in_order::Opts),
    #[structopt(
        about = "finds a positive integer k, if it exists, such that the sum of the digits of n taken to the successive powers of p is equal to k * n"
    )]
    DigPow(dig_pow::Opts),
}

fn main() {
    match Opts::from_args() {
        Opts::UniqueInOrder(opts) => {
            unique_in_order::main(opts);
        }
        Opts::DigPow(opts) => {
            dig_pow::main(opts);
        }
    }
}
