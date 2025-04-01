use super::ArgMatches;
use crate::loglib;
use crate::utilities;
use crate::dblib;

pub fn main(command: &ArgMatches) {
    let logger = loglib::Logger::new("save-password");
    match command.get_one::<String>("NAME") {
        Some(name) => {
            let password: String = utilities::input("Enter the password: ");
            if password.len() < 1 {
                logger.error("password must be at least one letter long!");
                return;
            }
            dblib::save_password(name.clone(), password);
        }
        _ => logger.error("<NAME> must be string!"), // It will not run..
    }
}