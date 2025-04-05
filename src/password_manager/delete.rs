use super::ArgMatches;
use crate::dblib;
use crate::loglib;
use crate::filelib;
use crate::errorlib;

pub fn main( command: &ArgMatches ) {
    let logger = loglib::Logger::new("delete-password");
    match command.get_one::<String>("ID") {
        Some(id) => {
            let pm_db_state = filelib::pm::db_state();
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
            let rows = dblib::delete_password(
                filelib::pm::get_decrypted_db_path(),
                id.clone()
            );
            logger.info(
                &format!("there is {} deleted.", rows)
            );
        },
        _ => logger.error(
            "<ID> must be integer!",
            errorlib::ExitErrorCode::UsageError
        ), // It will not run..
    }
}