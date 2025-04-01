use clap::ArgMatches;
use crate::password_manager::generate;
use crate::loglib;

pub fn matches(arg_matches: ArgMatches) {
    match arg_matches.subcommand() {
        Some(("password-manager", command)) => {
            match command.subcommand() {
                Some(("generate", command)) => generate::main(command),
                _ => loglib::error("Run with 'password-manager --help'")
            }
        },
        _ => loglib::error("Error: Run with '--help'")
    }
}