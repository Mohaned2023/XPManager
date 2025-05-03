use super::ArgMatches;
use crate::{
    dblib, 
    filelib, 
    loglib,
    errorlib
};

pub fn main( command: &ArgMatches ) {
    let logger = loglib::Logger::new("delete-log");
    if let Some(id) = command.get_one::<String>("ID") {
        let log_db_path = filelib::log::get_log_db_path();
        let log_db_state = filelib::get_file_state(
            log_db_path
                .to_str()
                .unwrap()
                .to_owned()
        );
        if log_db_state == filelib::FileState::NotFound {
            logger.error(
                "no logs database!", 
                errorlib::ExitErrorCode::FileNotFound
            );
        }
        let rows = dblib::log::delete_one(log_db_path, id.clone());
        logger.info(
            &format!("deleted {} log from the database.", rows)
        );
    }
}