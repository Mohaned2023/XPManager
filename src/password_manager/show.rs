use super::ArgMatches;
use crate::loglib;
use crate::filelib;
use crate::dblib;
use crate::displaylib;
use crate::errorlib;

pub fn main(command: &ArgMatches) {
    let logger = loglib::Logger::new("show-passwords");
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
    let passwords = dblib::get_passwords(
        filelib::get_pm_decrypted_db_path()
    );
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