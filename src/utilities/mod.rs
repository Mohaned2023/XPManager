use std::path::PathBuf;
use dirs::data_dir;
use crate::loglib;

#[derive(PartialEq)]
pub enum PasswordSample {
    Ascii,
    Hex
}
pub fn get_sample(sample: PasswordSample) -> Vec<char> {
    if sample == PasswordSample::Ascii {
        ('a'..='z')
        .chain('A'..='Z')
        .chain('0'..='9')
        .chain([
            '!', '@', '#', '$', '%', '^', '&', '(', ')', '-', '+', '=', '~',
            '[', ']', '{', '}', '/', '|', ':', ';', '?', ',', '.', '<', '>'
        ])
        .collect()
    } else {
        ('0'..='9')
        .chain('A'..='F')
        .collect()
    }
}

pub fn get_password_manager_db_path() -> PathBuf {
    let logger = loglib::Logger::new("password-manager-database");
    if let Some(data_path) = data_dir() {
        let xpm_data_path = data_path.join("XPManager/data");
        let de_file_path = xpm_data_path.join("passwords.db");
        let en_file_path = xpm_data_path.join("passwords.db.x");
        if !xpm_data_path.exists(){
            if let Err(_) = std::fs::create_dir_all(&xpm_data_path){
                logger.error("can NOT create the data directory!");
                panic!("Can NOT create the data directory!");
            }
            logger.info(
                &format!("create data directory at '{}'", xpm_data_path.display())
            );
        }
        if en_file_path.exists() { return en_file_path; }
        if !de_file_path.exists() {
            if let Err(_) = std::fs::File::create(&de_file_path) {
                logger.error("can NOT create the password manager database!");
                panic!("Can NOT create the password manager database!");
            }
            logger.info(
                &format!("create password manager database at '{}'", de_file_path.display())
            );
        } 
        return de_file_path;
    } else {
        logger.error("can NOT get the system data directory path!");
        panic!("Can NOT get the system data directory path!")
    }
}

pub fn is_encrypted(path: &PathBuf) -> bool {
    path.extension().unwrap() == "x"
}
