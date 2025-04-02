use std::path::PathBuf;
use dirs::data_dir;
use crate::{errorlib, loglib};

#[derive(PartialEq)]
pub enum FileState {
    Encrypted,
    Decrypted,
    NotFound
}

pub fn create_file(path: PathBuf) {
    let logger = loglib::Logger::new("create-file");
    if path.exists() {
        logger.info(
            &format!("file found at '{}'", path.display())
        );
        return;
    }
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            if let Err(_) = std::fs::create_dir_all(parent) {
                logger.error(
                    &format!("can NOT create directory at '{}'!", parent.display()),
                    errorlib::ExitErrorCode::CannotCreateDir
                );
            }
        }
    }
    if let Err(_) = std::fs::File::create(&path) {
        logger.error(
            &format!("can NOT create the at '{}'!", path.display()),
                errorlib::ExitErrorCode::CannotCreateFile
            );
    }
    logger.info(
        &format!("create password manager database at '{}'", path.display())
    );
}

pub fn get_pm_encrypted_db_path() -> PathBuf {
    let logger = loglib::Logger::new("get-pm-encrypted-db-path");
    if let Some(data_path) = data_dir() {
        return data_path.join("XPManager/data/passwords.db.x");
    } else {
        logger.error("can NOT get the system data directory path!", errorlib::ExitErrorCode::CannotAccessDataDir);
    }
}

pub fn get_pm_decrypted_db_path() -> PathBuf {
    let logger = loglib::Logger::new("get-pm-decrypted-db-path");
    if let Some(data_path) = data_dir() {
        return data_path.join("XPManager/data/passwords.db");
    } else {
        logger.error(
            "can NOT get the system data directory path!", 
            errorlib::ExitErrorCode::CannotAccessDataDir
        );
    }
}

pub fn password_manager_db_state() -> FileState {
    if get_pm_encrypted_db_path().exists() {
        return FileState::Encrypted;
    } else if get_pm_decrypted_db_path().exists() {
        return FileState::Decrypted;
    }
    return FileState::NotFound;
}