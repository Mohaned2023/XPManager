use super::ArgMatches;
use rand::seq::{IndexedRandom, SliceRandom};
use crate::errorlib;
use crate::filelib;
use crate::utilities;
use crate::loglib;
use crate::dblib;
use crate::displaylib;

fn generate(length: u16, sample_type: utilities::PasswordSample ) -> String {
    let mut rng = rand::rng();
    let mut sample = utilities::get_sample(sample_type);
    sample.shuffle(&mut rng);
    let mut password: String = String::new();
    for _ in 0..length {
        password.push(
            sample.choose(&mut rng)
                .unwrap()
                .clone()
        );
    }
    password
}

pub fn main(command: &ArgMatches) {
    let logger = loglib::Logger::new("generate-password");
    match command.get_one::<String>("length") {
        Some(length) => match length.parse::<u16>() {
            Ok(length) => {
                let mut _password: String = generate(
                    length,
                    if *command.get_one::<bool>("hex").unwrap() {
                        utilities::PasswordSample::Hex
                    } else {
                        utilities::PasswordSample::Ascii
                    }
                );
                logger.info("password generated successfully");
                if let Some(password_name) = command.get_one::<String>("save") {
                    let pm_db_state = filelib::password_manager_db_state();
                    let pm_decrypted_path = filelib::get_pm_decrypted_db_path();
                    if pm_db_state == filelib::FileState::NotFound {
                        filelib::create_file(pm_decrypted_path.clone());
                        dblib::create_passwords_table(pm_decrypted_path.clone());
                    } else if pm_db_state == filelib::FileState::Encrypted {
                        // TODO: decrypt the db.
                        // TODO: encrypt the db.
                        // TODO: secure delete the decrypt db.
                        todo!("pm database is encrypted!");
                    }
                    dblib::save_password(
                        pm_decrypted_path,
                        password_name.clone(),
                        _password.clone()
                    );
                }
                displaylib::passwords::display_one(_password);
            },
            Err(_) => logger.error(
                &format!("<LENGTH> must be unsigned integer from 0 to {}!", u16::MAX),
                errorlib::ExitErrorCode::UsageError
            ),
        },
        _ => logger.error(
            "Run with 'password-manager generate --help'",
            errorlib::ExitErrorCode::UsageError
        )
    }
}