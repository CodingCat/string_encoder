use std::env;

use base_x::encode;
extern crate base_x;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} input string", args[0]);
        std::process::exit(1);
    }
    let input_string = &args[1];
    let encoded = encode("0123456789abcdefghijklmnopqrstuvwxyz", input_string.as_bytes());
    println!("{}", encoded)
}