use super::ArgMatches;
use crate::loglib;
use crate::filelib;
use crate::dblib;
use crate::displaylib;

pub fn main(command: &ArgMatches) {
    let logger = loglib::Logger::new("find-password");
    match command.get_one::<String>("STRING") {
        Some(string) => {
            let pm_db_state = filelib::password_manager_db_state();
            if pm_db_state == filelib::FileState::NotFound {
                logger.error("password manager database is empty!");
                return;
            } else if pm_db_state == filelib::FileState::Encrypted {
                // TODO: decrypt the db.
                // TODO: encrypt the db.
                // TODO: secure delete the decrypt db.
                todo!("pm database is encrypted!");
            }
            let passwords = dblib::find_password(
                filelib::get_pm_decrypted_db_path(),
                string.clone()
            );
            displaylib::passwords::display_many(passwords, string.clone());
        },
        _ => logger.error("<STRING> must be string!"), // It will not run..
    }
}