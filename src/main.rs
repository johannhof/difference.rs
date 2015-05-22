extern crate text_diff;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    text_diff::print_diff(&args[1], &args[2], " ");
}

