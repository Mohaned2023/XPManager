
use super::{
    ArgMatches, 
    PMDatabaseEncrption
};
use crate::{
    loglib,
    filelib,
    errorlib,
    utilities
};

pub fn main(_: &ArgMatches) {
    let mut logger = loglib::Logger::new("decrypt-pm-database");
    let pm_db_state = filelib::pm::db_state();
    if pm_db_state == filelib::FileState::NotFound {
        logger.error(
            "no database, try to save some passwords!", 
            errorlib::ExitErrorCode::PMDatabaseNotFound
        );
    } else if pm_db_state == filelib::FileState::Decrypted {
        logger.error(
            "database not encrypted!", 
            errorlib::ExitErrorCode::FileNotEncrypted
        );
    }
    
    logger.warning("your passwords will be at risk if you decrypt the database!!");
    utilities::confirm();
    logger.start();
    let mut pm_db_encryption = PMDatabaseEncrption::new();
    pm_db_encryption.decrypt();
    logger.start();
    logger.warning("after you complete your work please encrypt your database!!");
    logger.info("password manager database decrypted successfully.");
}