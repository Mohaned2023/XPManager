use super::{
    ArgMatches,
    Fernet,
    Write,
    Read
};
use crate::{
    errorlib,
    filelib, 
    loglib,
    displaylib,
    utilities,
    dblib
};

pub fn encrypt(path: String, key: String) -> String {
    let logger = loglib::Logger::new("encrypt-file");
    let key = if key.len() < 1 {
        Fernet::generate_key()
    } else {
        key
    };
    if let Some(fernet) = Fernet::new(&key) {
        if let Ok(mut de_file) = std::fs::File::open(&path) {
            if let Ok(mut en_file) = std::fs::File::create(
                filelib::make_encrypt_path(path)
            ) {
                let mut buffer = vec![0u8;  64*1024]; // 64KB buffer.
                loop {
                    let bytes_read = de_file.read(&mut buffer).unwrap();
                    if bytes_read == 0 {
                        break;
                    }
                    let encryption_data = fernet.encrypt(&buffer[..bytes_read] );
                    let size = encryption_data.len() as u32;
                    en_file.write_all(&size.to_be_bytes()).unwrap();
                    en_file.write_all(&encryption_data.as_bytes()).unwrap();
                }
                return key;
            }
        }
        logger.error("can NOT open the file!", errorlib::ExitErrorCode::FileOpen);
    }
    logger.error("key error!", errorlib::ExitErrorCode::InvalidKey);
}

pub fn main(command: &ArgMatches) {
    let mut logger = loglib::Logger::new("encrypt-file");
    let path = command.get_one::<String>("PATH").unwrap();
    let is_key = *command.get_one::<bool>("key").unwrap_or(&false);
    let file_state = filelib::get_file_state(path.clone());
    if file_state == filelib::FileState::NotFound {
        logger.error(
            "file NOT found!",
            errorlib::ExitErrorCode::FileNotFound
        );
    }
    let mut _key = "".to_owned();
    if is_key {
        _key = utilities::input("Enter your key: ");
        logger.start();
    }
    logger.info("encryption in progress....");
    let key = encrypt(path.clone(), _key);
    if !is_key {
        displaylib::key::display(key);
        logger.warning("store the key somewhere safe!");
        logger.warning("if you lose the key, you will not be able to recover the data!");
    }
    logger.info("file encrypted successfully.");
    dblib::log::register(
        &format!("encrypt file at '{}'", path.clone()), 
        filelib::log::get_log_db_path()
    );
    if *command.get_one::<bool>("delete").unwrap_or(&false) {
        logger.start();
        filelib::wipe_delete(path.clone());
        logger.info("file wiped and deleted successfully.");
        dblib::log::register(
            &format!("file '{}' wiped", path.clone()), 
            filelib::log::get_log_db_path()
        );
    }
}


#[cfg(test)]
mod test {
    use std::path::PathBuf;
    use super::filelib::{create_file, delete_file};

    #[test]
    fn encrypt() {
        let temp_dir = PathBuf::new()
            .join("./temp/encrypt");
        let file = temp_dir.join("test.txt");
        let en_file = temp_dir.join("test.txt.x");
        create_file(file.clone());
        assert_eq!(file.exists(), true, "Can NOT create the test file!!");
        let file_path_str = file
            .to_str()
            .expect("Can NOT parse PathBuf to &str!!")
            .to_string();

        // without key
        let key = super::encrypt( file_path_str.clone(), "".to_string() );
        assert_eq!(key.len(), 44, "Key length error!");
        assert_eq!(en_file.exists(), true, "Can NOT encrypt the test file!!");

        // with key
        delete_file(en_file.clone());
        assert_eq!(en_file.exists(), false, "Can NOT delete the test file!!");
        let key = super::Fernet::generate_key();
        let old_key = super::encrypt(file_path_str.clone(), key.clone());
        assert_eq!(key, old_key, "Kay NOT match!!");
        assert_eq!(en_file.exists(), true, "Can NOT encrypt the test file!!");

        // delete temp files
        std::fs::remove_dir_all(temp_dir)
            .expect("Can NOT delete temp files!!");
    }
}