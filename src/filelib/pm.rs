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

#[cfg(test)]
mod tests {
    #[test]
    fn get_encrypted_db_path() {
        let db_path = super::get_encrypted_db_path();
        assert_eq!(
            db_path,
            super::data_dir()
                .expect("can NOT get the system data directory path!!")
                .join("XPManager/data/passwords.db.x"),
            "Encryption path NOT match!!"
        );
    }

    #[test]
    fn get_decrypted_db_path() {
        let db_path = super::get_decrypted_db_path();
        assert_eq!(
            db_path,
            super::data_dir()
                .expect("can NOT get the system data directory path!!")
                .join("XPManager/data/passwords.db"),
            "Encryption path NOT match!!"
        );
    }

    #[test]
    fn db_state() {
        let en_db_path = super::get_encrypted_db_path();
        let de_db_path = super::get_decrypted_db_path();
        let state = if en_db_path.exists() {
            super::FileState::Encrypted
        } else if de_db_path.exists() {
            super::FileState::Decrypted
        } else {
            super::FileState::NotFound
        };
        let result = state == super::db_state();
        assert_eq!( result, true, "Password manager database state NOT match!!" );
    }
}