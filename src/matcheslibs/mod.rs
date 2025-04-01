use clap::ArgMatches;
use crate::password_manager;
use crate::loglib;

pub fn matches(arg_matches: ArgMatches) {
    let logger = loglib::Logger::new("matches");
    match arg_matches.subcommand() {
        Some(("password-manager", command)) => {
            match command.subcommand() {
                Some(("generate", command)) => password_manager::generate::main(command),
                Some(("save", command)) =>     password_manager::save::main(command),
                _ => logger.error("Run with 'password-manager --help'")
            }
        },
        _ => logger.error("Error: Run with '--help'")
    }
}