mod lang;

use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        lang::print_help_menu();
        return;
    }
    match args[1].as_str() {
        "help" | "--help" => {
            lang::print_help_menu();
            return;
        },
        a => {
            lang::print_invalid_arg(a);
            lang::print_help_menu();
            return;
        }
    }
    println!("Hello, world!");
}
