use super::{
    ArgMatches,
    PMDatabaseEncrption
};
use crate::{
    loglib,
    filelib,
    dblib,
    errorlib,
    utilities
};

pub fn main(command: &ArgMatches) {
    let mut logger = loglib::Logger::new("update-password");
    let id = command.get_one::<String>("ID").unwrap();
    let mut _name: String = "".to_owned();
    let mut _password: String = "".to_owned();
    let mut _is_choose: bool = false;
    if *command.get_one::<bool>("name").unwrap_or(&false) {
        _name = utilities::input("Enter the name: ");
        logger.start();
        if _name.len() < 1 {
            logger.error(
                "name must be at least one letter long!",
                errorlib::ExitErrorCode::Input
            );
        }
        _is_choose = true;
    }
    if *command.get_one::<bool>("password").unwrap_or(&false) {
        _password = utilities::input("Enter the password: ");
        logger.start();
        if _password.len() < 1 {
            logger.error(
                "password must be at least one character long!",
                errorlib::ExitErrorCode::Input
            );
        }
        _is_choose = true;
    }
    if !_is_choose {
        logger.error(
            "Run with 'password-manager update --help'",
            errorlib::ExitErrorCode::MissingArg
        )
    }
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
    let pm_db_path = filelib::pm::get_decrypted_db_path();
    if _password.len() > 0 {
        let rows = dblib::pm::update_password(
            pm_db_path.clone(), 
            id.clone(), 
            _password
        );
        if rows > 0 {
            dblib::log::register(
                &format!("the name of the password with id {} updated", id),
                filelib::log::get_log_db_path()
            );
        }
        logger.info(
            &format!("there is {} password updated successfully.", rows)
        );
    }
    if _name.len() > 0 {
        let rows = dblib::pm::update_password_name(
            pm_db_path, 
            id.clone(), 
            _name
        );
        if rows > 0 {
            dblib::log::register(
                &format!("password with id {} updated", id),
                filelib::log::get_log_db_path()
            );
        }
        logger.info(
            &format!("there is {} password name update successfully.", rows)
        );
    }
    if _is_db_decrypted {
        pm_db_encryption.encrypt();
        logger.info("password manager database encrypted successfully.");
    }
}