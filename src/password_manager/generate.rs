use clap::ArgMatches;
use rand::seq::{IndexedRandom, SliceRandom};
use crate::utilities;

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
        Some(length) => {
            let length: u16 = length
                .parse()
                .expect(
                    &format!("<LENGTH> must be unsigned integer from 0 to {}!", u16::MAX)
                );
            let mut _password: String = generate(
                length,
                if *command.get_one::<bool>("hex").unwrap() {
                    utilities::PasswordSample::Hex
                } else {
                    utilities::PasswordSample::Ascii
                }
            );
            // TODO: Use the loglib.
            println!("password: {}", _password);
            if let Some(password_name) = command.get_one::<String>("save") {
                // TODO: Save the password.
            }
        },
        _ => eprintln!("Error: Run with 'password-manager generate --help'")
    }
}