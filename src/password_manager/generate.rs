use clap::ArgMatches;
use rand::seq::{IndexedRandom, SliceRandom};
use crate::utilities;
use crate::loglib;
use chrono::Local;

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
    match command.get_one::<String>("length") {
        Some(length) => match length.parse::<u16>() {
            Ok(length) => {
                let start_time = Local::now().timestamp_millis();
                let mut _password: String = generate(
                    length,
                    if *command.get_one::<bool>("hex").unwrap() {
                        utilities::PasswordSample::Hex
                    } else {
                        utilities::PasswordSample::Ascii
                    }
                );
                loglib::password_manager::password(_password);
                let delay_ms = Local::now().timestamp_millis()-start_time;
                loglib::info("password generated successfully", delay_ms);
                if let Some(password_name) = command.get_one::<String>("save") {
                    // TODO: Save the password.
                }
            },
            Err(_) => loglib::error(
                &format!("<LENGTH> must be unsigned integer from 0 to {}!", u16::MAX)
            ),
        },
        _ => loglib::error("Run with 'password-manager generate --help'")
    }
}