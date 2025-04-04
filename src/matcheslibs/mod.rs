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
                Some(("show", command)) =>     password_manager::show::main(command),
                Some(("update", command)) =>   password_manager::update::main(command),
                Some(("count", command)) =>    password_manager::count::main(command),
                Some(("delete", command)) =>   password_manager::delete::main(command),
                _ => logger.error(
                    "Run with 'password-manager --help'",
                    errorlib::ExitErrorCode::UsageError
                )
            }
        },
        _ => logger.error(
            "Run with '--help'",
            errorlib::ExitErrorCode::UsageError
        )
    }
}