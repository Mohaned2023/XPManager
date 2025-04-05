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
    utilities
};

pub fn decrypt(path: String, key: String) {
    let logger = loglib::Logger::new("decrypt-file");
    if let Ok(mut en_file) = std::fs::File::open(&path) {
        if let Ok(mut de_file) = std::fs::File::create(
            filelib::make_decrypt_path(path)
        ) {
            if let Some(fernet) = Fernet::new(&key) {
                let mut size_buf = [0u8; 4];
                loop {
                    if en_file.read_exact(&mut size_buf).is_err() {
                        break;
                    }
                    let size = u32::from_be_bytes(size_buf) as usize;
                    let mut encryption_buffer = vec![0u8; size];
                    en_file.read_exact(&mut encryption_buffer).unwrap();
                    let data = fernet.decrypt(
                        &String::from_utf8(
                            encryption_buffer
                        ).unwrap()
                    ).unwrap();
                    de_file.write_all(&data).unwrap();
                }
                return;
            }
            logger.error("key error!", errorlib::ExitErrorCode::NoDataAvilable);
        }
    }
    logger.error("can NOT open the file!", errorlib::ExitErrorCode::FileNotFound);
}

pub fn main(command: &ArgMatches) {
    let mut logger = loglib::Logger::new("decrypt-file");
    if let Some(path) = command.get_one::<String>("PATH") {
        let file_state = filelib::get_file_state(path.clone());
        if file_state == filelib::FileState::NotFound {
            logger.error(
                "file NOT exists!",
                errorlib::ExitErrorCode::FileNotFound
            );
        } else if file_state == filelib::FileState::Decrypted {
            logger.error(
                "file NOT encrpted!",
                errorlib::ExitErrorCode::NoDataAvilable
            );
        }
        let key =  utilities::input("Enter your key: ");
        logger.start();
        logger.info("decryption in progress....");
        decrypt(path.clone(), key);
        logger.info("file decrypted successfully.");
        
        if *command.get_one::<bool>("delete").unwrap_or(&false) {
            logger.start();
            filelib::wipe_delete(path.clone());
            logger.info("file wiped and deleted successfully.");
        }
    }
}