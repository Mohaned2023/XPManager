use super::ArgMatches;
use crate::loglib;
use crate::utilities;
use crate::dblib;
use crate::filelib;

pub fn main(command: &ArgMatches) {
    let logger = loglib::Logger::new("save-password");
    match command.get_one::<String>("NAME") {
        Some(name) => {
            let password: String = utilities::input("Enter the password: ");
            if password.len() < 1 {
                logger.error("password must be at least one letter long!");
                return;
            }
            let pm_db_state = filelib::password_manager_db_state();
            if pm_db_state == filelib::FileState::NotFound {
                filelib::create_file(
                    filelib::get_pm_decrypted_db_path()
                );
            } else if pm_db_state == filelib::FileState::Encrypted {
                // TODO: decrypt the db.
                // TODO: encrypt the db.
                // TODO: secure delete the decrypt db.
                todo!("pm database is encrypted!");
            }
            dblib::save_password(
                filelib::get_pm_decrypted_db_path(),
                name.clone(),
                password
            );
        }
        _ => logger.error("<NAME> must be string!"), // It will not run..
    }
}