
use super::{
    ArgMatches, 
    PMDatabaseEncrption
};
use crate::{
    loglib,
    filelib,
    errorlib
};

pub fn main(_: &ArgMatches) {
    let mut logger = loglib::Logger::new("decrypt-pm-database");
    let pm_db_state = filelib::pm::db_state();
    if pm_db_state == filelib::FileState::NotFound {
        logger.error(
            "no database, try to save some passwords!", 
            errorlib::ExitErrorCode::FileNotFound
        );
    }
    let mut pm_db_encryption = PMDatabaseEncrption::new();
    pm_db_encryption.decrypt();
    logger.start();
    logger.warning("your passwords will be at risk!");
    logger.warning("after you complet your work please encrypt your database!!");
    logger.info("database decrypted successfully.");
}