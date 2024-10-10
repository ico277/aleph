mod argparse;

use std::env::args;
use argparse::test;

fn main() {
    let args: Vec<String> = args().collect();
    match args[1].as_str() {
        "help" | "--help" => {
            println!("HELP MENU HERE");
            test();
        },
        a => {
            eprintln!("Error - Invalid argument {}!", a);
            return;
        }
    }
    println!("Hello, world!");
}
