use super::ArgMatches;
use super::PMDatabaseEncrption;
use crate::loglib;
use crate::filelib;
use crate::dblib;
use crate::displaylib;
use crate::errorlib;

pub fn main(command: &ArgMatches) {
    let mut logger = loglib::Logger::new("find-password");
    match command.get_one::<String>("STRING") {
        Some(string) => {
            let pm_db_state = filelib::pm::db_state();
            let mut pm_db_encryption = PMDatabaseEncrption::new();
            let mut _is_db_decrypted: bool = false;
            if pm_db_state == filelib::FileState::NotFound {
                logger.error(
                    "password manager database is empty!",
                    errorlib::ExitErrorCode::NoDataAvilable
                );
            } else if pm_db_state == filelib::FileState::Encrypted {
                logger.warning("database encrypted!");
                pm_db_encryption.decrypt();
                logger.start();
                _is_db_decrypted = true;
                logger.info("password manager database decrypted successfully.");
            }
            let passwords = dblib::find_password(
                filelib::pm::get_decrypted_db_path(),
                string.clone()
            );
            if _is_db_decrypted {
                pm_db_encryption.encrypt();
                logger.info("password manager database encrypted successfully.");
            }
            displaylib::passwords::display_many(passwords, string.clone());
        },
        _ => logger.error(
            "<STRING> must be string!",
            errorlib::ExitErrorCode::UsageError
        ), // It will not run..
    }
}