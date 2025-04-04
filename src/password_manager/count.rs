use super::ArgMatches;
use crate::dblib;
use crate::loglib;
use crate::filelib;
use crate::errorlib;

pub fn main( _: &ArgMatches ) {
    let logger = loglib::Logger::new("count-password");
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
    let number_of_passwords = dblib::get_passwords_number(
        filelib::get_pm_decrypted_db_path()
    );
    logger.info(
        &format!("there is {} in the database.", number_of_passwords)
    );
}