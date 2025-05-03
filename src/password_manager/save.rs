use super::{
    ArgMatches,
    PMDatabaseEncrption
};
use crate::{
    errorlib,
    loglib,
    utilities,
    dblib,
    filelib
};

pub fn main(command: &ArgMatches) {
    let mut logger = loglib::Logger::new("save-password");
    let name = command.get_one::<String>("NAME").unwrap();
    let password: String = utilities::input("Enter the password: ");
    if password.len() < 1 {
        logger.error(
            "password must be at least one character long!",
            errorlib::ExitErrorCode::Input
        );
    }
    let pm_db_state = filelib::pm::db_state();
    let mut pm_db_encryption = PMDatabaseEncrption::new();
    let mut _is_db_decrypted: bool = false;
    if pm_db_state == filelib::FileState::NotFound {
        filelib::create_file(
            filelib::pm::get_decrypted_db_path()
        );
    } else if pm_db_state == filelib::FileState::Encrypted {
        logger.warning("database is encrypted!");
        pm_db_encryption.decrypt();
        logger.start();
        _is_db_decrypted = true;
        logger.info("password manager database decrypted successfully.");
    }
    dblib::pm::save_password(
        filelib::pm::get_decrypted_db_path(),
        name.clone(),
        password
    );
    if _is_db_decrypted {
        pm_db_encryption.encrypt();
        logger.info("password manager database encrypted successfully.");
    }
}