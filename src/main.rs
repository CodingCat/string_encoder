use std::env;

extern crate base_x;
use base_x::encode;

fn encode_string_with_base_36(input_str: &str) -> String {
    return encode("0123456789abcdefghijklmnopqrstuvwxyz", input_str.as_bytes());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} input_string", args[0]);
        std::process::exit(1);
    }
    let input_string = &args[1];
    let encoded = encode_string_with_base_36(input_string);
    println!("{}", encoded)
}

#[cfg(test)]
mod test;
