use clap::ArgMatches;
use crate::password_manager;
use crate::encryption_manager;
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
                Some(("encrypt", command)) =>  password_manager::encrypt::main(command),
                Some(("decrypt", command)) =>  password_manager::decrypt::main(command),
                _ => logger.error(
                    "Run with 'password-manager --help'",
                    errorlib::ExitErrorCode::UsageError
                )
            }
        },
        Some(("encryption-manager", command)) => {
            match command.subcommand() {
                Some(("encrypt-file", command)) => encryption_manager::encrypt_file::main(command),
                Some(("decrypt-file", command)) => encryption_manager::decrypt_file::main(command),
                Some(("encrypt-dir", command)) => encryption_manager::encrypt_dir::main(command),
                Some(("decrypt-dir", command)) => encryption_manager::decrypt_dir::main(command),
                Some(("encode", command)) => encryption_manager::encode::main(command),
                Some(("decode", command)) => encryption_manager::decode::main(command),
                _ => logger.error(
                    "Run with 'encryption-manager --help'",
                    errorlib::ExitErrorCode::UsageError
                )
            }
        }
        _ => logger.error(
            "Run with '--help'",
            errorlib::ExitErrorCode::UsageError
        )
    }
}