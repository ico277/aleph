use std::env::args;

const HELP: &str = include_str!("../lang/menus/help");
const ERROR_INVALID_ARG: &str = include_str!("../lang/errors/invalid_arg");

pub fn print_help_menu() {
    let cmd = args().next().unwrap_or("aleph".to_string());
    let help = HELP.replace("%CMD%", cmd.as_str());
    println!("{}", help)
}

pub fn print_invalid_arg<S>(arg: S) where S: AsRef<str> {
    let arg = arg.as_ref();
    let error_invalid_arg = ERROR_INVALID_ARG.replace("%ARG%", arg);
    eprintln!("{}", error_invalid_arg);
}
