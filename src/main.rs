use naknak::{decode, encode};
use std::env;

fn help() {
    println!("Usage: -e STRING_TO_ENCODE");
    println!("       -d STRING_TO_DECODE");
    println!("       -h Print this help");
}

fn main() {
    let args = env::args().skip(1).collect::<Vec<String>>();
    if args.len() == 0 {
        help();
        return;
    }
    for arg in args.as_slice().chunks_exact(2) {
        if &arg[0] == "-h" {
            help();
            return;
        }
        if &arg[0] == "-e" {
            println!("{}", encode(&arg[1]));
        }
        if &arg[0] == "-d" {
            match decode(&arg[1]) {
                Ok(dec) => {
                    println!("{}", dec);
                }
                Err(e) => {
                    println!("Error decoding string: {}", e);
                }
            }
        }
    }
}
