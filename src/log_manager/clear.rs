use super::ArgMatches;
use crate::{
    dblib, 
    filelib, 
    loglib,
    errorlib
};

pub fn main( _: &ArgMatches ) {
    let logger = loglib::Logger::new("clear-log");
    let log_db_path = filelib::log::get_log_db_path();
    let log_db_state = filelib::get_file_state(
        log_db_path
            .to_str()
            .unwrap()
            .to_owned()
    );
    if log_db_state == filelib::FileState::NotFound {
        logger.error("no logs database!", errorlib::ExitErrorCode::FileNotFound);
    }
    logger.warning("clear all logs!");
    let rows = dblib::log::delete_all(log_db_path);
    logger.info(
        &format!("clear {} log from the database.", rows)
    );
}