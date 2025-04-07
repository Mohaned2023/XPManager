use super::ArgMatches;
use crate::{
    dblib, 
    filelib, 
    loglib,
    errorlib,
    displaylib
};


pub fn main( command: &ArgMatches ) {
    let logger = loglib::Logger::new("find-log");
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
    match command.subcommand() {
        Some(("string", command)) => {
            let string = command.get_one::<String>("STRING").unwrap();
            displaylib::log::display(
                dblib::log::get_logs(
                    log_db_path, 
                    0, 
                    string.clone()
                )
            );
        },
        Some(("date", command)) => {
            let logs = dblib::log::get_logs_by_date(
                log_db_path, 
                (
                    // year
                    command.get_one::<String>("year")
                        .unwrap_or(&"0".to_owned())
                        .parse::<u16>()
                        .unwrap_or(0),
                    // month
                    command.get_one::<String>("month")
                        .unwrap_or(&"0".to_owned())
                        .parse::<u8>()
                        .unwrap_or(0),
                    // day
                    command.get_one::<String>("day")
                        .unwrap_or(&"0".to_owned())
                        .parse::<u8>()
                        .unwrap_or(0)
                )
            );
            if logs.len() < 1 {
                logger.error(
                    "no logs found!", 
                    errorlib::ExitErrorCode::NoDataAvilable
                )
            }
            displaylib::log::display( logs );
            logger.info("logs displayed successfully.");
        },
        _ => {}
    }
}