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

pub fn main( _: &ArgMatches ) {
    let mut logger = loglib::Logger::new("count-password");
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
    let number_of_passwords = dblib::pm::get_passwords_number(
        filelib::pm::get_decrypted_db_path()
    );
    if _is_db_decrypted {
        pm_db_encryption.encrypt();
        logger.info("password manager database encrypted successfully.");
    }
    logger.info(
        &format!("there is {} in the database.", number_of_passwords)
    );
}