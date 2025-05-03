use super::ArgMatches;
use crate::{
    dblib, 
    filelib, 
    loglib,
    errorlib,
    displaylib
};

pub fn main( command: &ArgMatches ) {
    let logger = loglib::Logger::new("show-log");
    let log_db_path = filelib::log::get_log_db_path();
    let log_db_state = filelib::get_file_state(
        log_db_path
            .to_str()
            .unwrap()
            .to_owned()
    );
    let length = command.get_one::<String>("length")
        .unwrap_or(&"0".to_owned())
        .parse::<u16>();
    if log_db_state == filelib::FileState::NotFound {
        logger.error("no logs database!", errorlib::ExitErrorCode::LMDatabaseNotFound);
    }
    displaylib::log::display(
        dblib::log::get_logs(
            log_db_path, 
            length.unwrap_or(0u16), 
            "".to_owned()
        )
    );
}