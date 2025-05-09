use super::{
    XPM_EXTENSION,
    data_dir,
    PathBuf,
    loglib,
    errorlib,
    FileState
};

pub fn get_encrypted_db_path() -> PathBuf {
    let logger = loglib::Logger::new("get-pm-encrypted-db-path");
    if let Some(data_path) = data_dir() {
        return data_path.join(
            format!("XPManager/data/passwords.db.{}", XPM_EXTENSION)
        );
    } else {
        logger.error(
            "can NOT get the system data directory path!",
            errorlib::ExitErrorCode::SystemDataDirNotFound
        );
    }
}

pub fn get_decrypted_db_path() -> PathBuf {
    let logger = loglib::Logger::new("get-pm-decrypted-db-path");
    if let Some(data_path) = data_dir() {
        return data_path.join("XPManager/data/passwords.db");
    } else {
        logger.error(
            "can NOT get the system data directory path!", 
            errorlib::ExitErrorCode::SystemDataDirNotFound
        );
    }
}

pub fn db_state() -> FileState {
    if get_encrypted_db_path().exists() {
        return FileState::Encrypted;
    } else if get_decrypted_db_path().exists() {
        return FileState::Decrypted;
    }
    return FileState::NotFound;
}

pub fn warning_encrypt_database() {
    let logger = loglib::Logger::new("check-password-manager-database");
    if db_state() == FileState::Decrypted {
        logger.warning("password manager database found NOT encrypted!!");
        logger.warning("please use 'password-manager encrypt' to encrypt it!");
    }
}