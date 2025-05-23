use super::{
    ArgMatches,
    PMDatabaseEncrption
};
use crate::{
    dblib,
    loglib,
    filelib,
    errorlib
};

pub fn main( command: &ArgMatches ) {
    let mut logger = loglib::Logger::new("delete-password");
    let id = command.get_one::<String>("ID").unwrap();
    let pm_db_state = filelib::pm::db_state();
    let mut pm_db_encryption = PMDatabaseEncrption::new();
    let mut _is_db_decrypted: bool = false;
    if pm_db_state == filelib::FileState::NotFound {
        logger.error(
            "password manager database is empty!",
            errorlib::ExitErrorCode::PMDatabaseNotFound
        );
    } else if pm_db_state == filelib::FileState::Encrypted {
        logger.warning("database is encrypted!");
        pm_db_encryption.decrypt();
        logger.start();
        _is_db_decrypted = true;
        logger.info("password manager database decrypted successfully.");
    }
    let rows = dblib::pm::delete_password(
        filelib::pm::get_decrypted_db_path(),
        id.clone()
    );
    if rows > 0 {
        dblib::log::register(
            &format!("password with id {} deleted", id),
            filelib::log::get_log_db_path()
        );
    }
    if _is_db_decrypted {
        pm_db_encryption.encrypt();
        logger.info("password manager database encrypted successfully.");
    }
    logger.info(
        &format!("there is {} password deleted.", rows)
    );
}