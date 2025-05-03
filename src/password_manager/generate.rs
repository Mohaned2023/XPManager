use super::{
    ArgMatches,
    PMDatabaseEncrption
};
use rand::seq::{
    IndexedRandom, 
    SliceRandom
};
use crate::{
    errorlib,
    filelib,
    utilities,
    loglib,
    dblib,
    displaylib
};

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
    let mut logger = loglib::Logger::new("generate-password");
    let length = command
        .get_one::<String>("LENGTH")
        .unwrap()
        .parse::<u16>();
    if length.is_err() {
        logger.error(
            &format!("<LENGTH> must be unsigned integer from 0 to {}!", u16::MAX),
            errorlib::ExitErrorCode::Usage
        )
    }
    let mut _password: String = generate(
        length.unwrap(),
        if *command.get_one::<bool>("hex").unwrap() {
            utilities::PasswordSample::Hex
        } else {
            utilities::PasswordSample::Ascii
        }
    );
    logger.info("password generated successfully.");
    if let Some(password_name) = command.get_one::<String>("save") {
        let pm_db_state = filelib::pm::db_state();
        let mut pm_db_encryption = PMDatabaseEncrption::new();
        let mut _is_db_decrypted: bool = false;
        let pm_decrypted_path = filelib::pm::get_decrypted_db_path();
        if pm_db_state == filelib::FileState::NotFound {
            filelib::create_file(pm_decrypted_path.clone());
            dblib::pm::create_passwords_table(pm_decrypted_path.clone());
        } else if pm_db_state == filelib::FileState::Encrypted {
            logger.warning("database is encrypted!");
            pm_db_encryption.decrypt();
            logger.start();
            _is_db_decrypted = true;
            logger.info("password manager database decrypted successfully.");
        }
        dblib::pm::save_password(
            pm_decrypted_path,
            password_name.clone(),
            _password.clone()
        );
        if _is_db_decrypted {
            pm_db_encryption.encrypt();
            logger.info("password manager database encrypted successfully.");
        }
    }
    displaylib::passwords::display_one(_password);
}