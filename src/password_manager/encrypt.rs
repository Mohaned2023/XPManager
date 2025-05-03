
use super::{
    ArgMatches, 
    PMDatabaseEncrption
};
use crate::{
    displaylib, 
    errorlib, 
    filelib, 
    loglib
};
use fernet::Fernet;

pub fn main(command: &ArgMatches) {
    let mut logger = loglib::Logger::new("encrypt-pm-database");
    let pm_db_state = filelib::pm::db_state();
    if pm_db_state == filelib::FileState::NotFound {
        logger.error(
            "no database, try to save some passwords and then encrypt it!", 
            errorlib::ExitErrorCode::PMDatabaseNotFound
        );
    } else if pm_db_state == filelib::FileState::Encrypted {
        logger.error(
            "database is already encrypted!", 
            errorlib::ExitErrorCode::FileAlreadyEncrypted
        );
    }
    let mut pm_db_encryption = PMDatabaseEncrption::new();
    if *command.get_one::<bool>("key").unwrap_or(&false) {
        pm_db_encryption.set_key(None);
        logger.start();
    } else { 
        let key = Fernet::generate_key();
        pm_db_encryption.set_key(
            Some(key.clone())
        );
        displaylib::key::display(key);
    }
    pm_db_encryption.encrypt();
    logger.info("database encrypted successfully.");
}