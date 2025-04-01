use clap::ArgMatches;
use rand::seq::{IndexedRandom, SliceRandom};
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
    let logger = loglib::Logger::new("gernerate-password");
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
                displaylib::passwords::display_one(_password.clone());
                logger.info("password generated successfully");
                if let Some(password_name) = command.get_one::<String>("save") {
                    dblib::save_password(password_name.clone(), _password);
                }
            },
            Err(_) => logger.error(
                &format!("<LENGTH> must be unsigned integer from 0 to {}!", u16::MAX)
            ),
        },
        _ => logger.error("Run with 'password-manager generate --help'")
    }
}