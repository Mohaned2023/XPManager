use super::ArgMatches;
use super::PMDatabaseEncrption;
use crate::dblib;
use crate::loglib;
use crate::filelib;
use crate::errorlib;

pub fn main( command: &ArgMatches ) {
    let mut logger = loglib::Logger::new("delete-password");
    match command.get_one::<String>("ID") {
        Some(id) => {
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
            let rows = dblib::delete_password(
                filelib::pm::get_decrypted_db_path(),
                id.clone()
            );
            if _is_db_decrypted {
                pm_db_encryption.encrypt();
                logger.info("password manager database encrypted successfully.");
            }
            logger.info(
                &format!("there is {} deleted.", rows)
            );
        },
        _ => logger.error(
            "<ID> must be integer!",
            errorlib::ExitErrorCode::UsageError
        ), // It will not run..
    }
}