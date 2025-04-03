use super::ArgMatches;
use crate::loglib;
use crate::filelib;
use crate::dblib;
use crate::errorlib;
use crate::utilities;

pub fn main(command: &ArgMatches) {
    let mut logger = loglib::Logger::new("update-password");
    match command.get_one::<String>("ID") {
        Some(id) => {
            let mut _name: String = "".to_owned();
            let mut _password: String = "".to_owned();
            let mut _is_choose: bool = false;
            if *command.get_one::<bool>("name").unwrap_or(&false) {
                _name = utilities::input("Enter the name: ");
                logger.start();
                if _name.len() < 1 {
                    logger.error(
                        "name must be at least one letter long!",
                        errorlib::ExitErrorCode::UsageError
                    );
                }
                _is_choose = true;
            }
            if *command.get_one::<bool>("password").unwrap_or(&false) {
                _password = utilities::input("Enter the password: ");
                logger.start();
                if _password.len() < 1 {
                    logger.error(
                        "password must be at least one letter long!",
                        errorlib::ExitErrorCode::UsageError
                    );
                }
                _is_choose = true;
            }
            if !_is_choose {
                logger.error(
                    "Run with 'password-manager update --help'",
                    errorlib::ExitErrorCode::UsageError
                )
            }
            let pm_db_state = filelib::password_manager_db_state();
            if pm_db_state == filelib::FileState::NotFound {
                logger.error(
                    "password manager database is empty!",
                    errorlib::ExitErrorCode::NoDataAvilable
                );
            } else if pm_db_state == filelib::FileState::Encrypted {
                // TODO: decrypt the db.
                // TODO: encrypt the db.
                // TODO: secure delete the decrypt db.
                todo!("pm database is encrypted!");
            }
            let pm_db_path = filelib::get_pm_decrypted_db_path();
            if _password.len() > 0 {
                dblib::update_password(
                    pm_db_path.clone(), 
                    id.clone(), 
                    _password
                );
                logger.info("password updated successfully.");
            }
            if _name.len() > 0 {
                dblib::update_password_name(
                    pm_db_path, 
                    id.clone(), 
                    _name
                );
                logger.info("name update successfully.");
            }
        }
        _ => logger.error(
            "<ID> must be integer!",
            errorlib::ExitErrorCode::UsageError
        ), // It will not run..
    }

}