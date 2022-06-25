use structopt::StructOpt;

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
}

fn main() {
    match Opts::from_args() {
        Opts::UniqueInOrder(opts) => {
            unique_in_order::main(opts);
        }
    }
}
