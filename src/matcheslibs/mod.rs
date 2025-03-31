use clap::ArgMatches;
use super::password_manager::generate;

pub fn matches(arg_matches: ArgMatches) {
    match arg_matches.subcommand() {
        Some(("password-manager", command)) => {
            match command.subcommand() {
                Some(("generate", command)) => generate::main(command),
                _ => eprintln!("Error: Run with 'password-manager --help'")
            }
        },
        _ => eprintln!("Error: Run with '--help'")
    }
}