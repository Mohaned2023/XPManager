use super::{
    ArgMatches,
    PMDatabaseEncrption
};
use crate::{
    loglib,
    filelib,
    dblib,
    displaylib,
    errorlib
};

pub fn main(command: &ArgMatches) {
    let mut logger = loglib::Logger::new("show-passwords");
    let pm_db_state = filelib::pm::db_state();
    let mut pm_db_encryption = PMDatabaseEncrption::new();
    let mut _is_db_decrypted: bool = false;
    if pm_db_state == filelib::FileState::NotFound {
        logger.error(
            "password manager database is empty!",
            errorlib::ExitErrorCode::NoDataAvilable
        );
    } else if pm_db_state == filelib::FileState::Encrypted {
        logger.warning("database is encrypted!");
        pm_db_encryption.decrypt();
        logger.start();
        _is_db_decrypted = true;
        logger.info("password manager database decrypted successfully.");
    }
    let passwords = dblib::pm::get_passwords(
        filelib::pm::get_decrypted_db_path()
    );
    if _is_db_decrypted {
        pm_db_encryption.encrypt();
        logger.info("password manager database encrypted successfully.");
    }
    if passwords.len() < 1 {
        logger.error(
            "password manager database is empty!",
            errorlib::ExitErrorCode::NoDataAvilable
        );
    }
    if *command.get_one::<bool>("table").unwrap_or(&false) {
        displaylib::passwords::display_as_table(passwords);
    } else {
        displaylib::passwords::display_many(passwords, "".to_owned());
    }
    logger.info("all passwords have been successfully displayed.");
}