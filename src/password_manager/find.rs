use super::ArgMatches;
use crate::loglib;
use crate::filelib;
use crate::dblib;
use crate::displaylib;
use crate::errorlib;

pub fn main(command: &ArgMatches) {
    let logger = loglib::Logger::new("find-password");
    match command.get_one::<String>("STRING") {
        Some(string) => {
            let pm_db_state = filelib::pm::db_state();
            if pm_db_state == filelib::FileState::NotFound {
                logger.error(
                    "password manager database is empty!",
                    errorlib::ExitErrorCode::NoDataAvilable
                );
            } else if pm_db_state == filelib::FileState::Encrypted {
                // TODO: decrypt the db.
                // TODO: encrypt the db.
                // TODO: secure delete the decrypt db.
                todo!("pm database is encrypted!");
            }
            let passwords = dblib::find_password(
                filelib::pm::get_decrypted_db_path(),
                string.clone()
            );
            displaylib::passwords::display_many(passwords, string.clone());
        },
        _ => logger.error(
            "<STRING> must be string!",
            errorlib::ExitErrorCode::UsageError
        ), // It will not run..
    }
}