use clap::ArgMatches;
use crate::password_manager;
use crate::loglib;
use crate::errorlib;

pub fn matches(arg_matches: ArgMatches) {
    let logger = loglib::Logger::new("matches");
    match arg_matches.subcommand() {
        Some(("password-manager", command)) => {
            match command.subcommand() {
                Some(("generate", command)) => password_manager::generate::main(command),
                Some(("save", command)) =>     password_manager::save::main(command),
                Some(("find", command)) =>     password_manager::find::main(command),
                _ => logger.error(
                    "Run with 'password-manager --help'",
                    errorlib::ExitErrorCode::UsageError
                )
            }
        },
        _ => logger.error(
            "Error: Run with '--help'",
            errorlib::ExitErrorCode::UsageError
        )
    }
}