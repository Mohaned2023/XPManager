pub mod pm;

use std::ffi::OsStr;
use std::path::{PathBuf, Path};
use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom, Write};
use dirs::data_dir;
use rand::Rng;
use crate::{errorlib, loglib};

const XPM_EXTENSION: &str = "x";

#[derive(PartialEq)]
pub enum FileState {
    Encrypted,
    Decrypted,
    NotFound
}

#[derive(PartialEq)]
enum WipeType {
    BZero,
    BOne,
    Random
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

pub fn delete_file(path: PathBuf) {
    let logger = loglib::Logger::new("delete-file");
    if path.exists() {
        if let Err(_) = std::fs::remove_file(&path) {
            logger.error(
                &format!("can NOT delete file at '{}'!", path.display()),
                errorlib::ExitErrorCode::CanNotDeleteFile
            );
        }
    }
}

fn wipe_file(path: String, wipe_type: WipeType) {
    let logger = loglib::Logger::new("wipe-file");
    let path = Path::new(&path);
    if !path.exists() || !path.is_file() {
        logger.error(
            "file not found!", 
            errorlib::ExitErrorCode::NoDataAvilable
        );
    }
    if let Ok(mut file) = OpenOptions::new()
        .write(true) 
        .open(path) {
        if let Ok(metadata) = file.metadata() {
            let len = metadata.len();
            let mut size: usize = 64*1024; // 64KB.
            size = if len < size as u64 {
                // if the size of the file is less than 64KB.
                len as usize
            } else { size };
            let mut pos= 0u64;
            let mut rng = rand::rng();
            // Make the data vec based on the wipe type.
            let data = if wipe_type == WipeType::Random {
                // Make a static rng for all buffers.
                // When it is a static rng the speed is up!
                let mut data = vec![0u8; size];
                rng.fill(&mut data[..]);
                data
            } else if wipe_type == WipeType::BOne {
                vec![1u8; size]
            } else {
                vec![0u8; size]
            };
            loop {
                if pos + size as u64 > len && pos < len {
                    // if len = 65KB and pos = 64KB we have 1KB to be
                    // written. to write this 1KB: len - pos = 1KB 
                    // We will use this as the size of the buffer.
                    size = (len - pos) as usize;
                } 
                if pos > len { break; }
                if let Err(_) = file.seek(SeekFrom::Start(pos)) {
                    logger.error(
                        "can NOT seek the file!", 
                        errorlib::ExitErrorCode::NoDataAvilable
                    );
                }
                if let Err(_) = file.write_all(&data) {
                    logger.error(
                        "can NOT write to the file!", 
                        errorlib::ExitErrorCode::NoDataAvilable
                    );
                }
                pos += size as u64;
            }
            if let Err(_) = file.flush() {
                logger.error(
                    "can NOT flush the file to the disk!", 
                    errorlib::ExitErrorCode::NoDataAvilable
                );
            }
        }
    }
}

pub fn wipe_delete(path: String) {
    // We will use 4 levels wiping:
    wipe_file(path.clone(), WipeType::BOne);   // L1: with 1s.
    wipe_file(path.clone(), WipeType::Random); // L2: with static random data.
    wipe_file(path.clone(), WipeType::Random); // L3: with static random data.
    wipe_file(path.clone(), WipeType::BZero);  // L4: with 0s.
    delete_file(PathBuf::new().join(path));
}

pub fn get_file_state(path: String) -> FileState {
    let path = std::path::Path::new(&path);
    let mut _state: FileState;
    if path.extension()
        .unwrap_or(&OsStr::new(""))
        .to_str()
        .unwrap() == XPM_EXTENSION {
            _state = FileState::Encrypted;
    } else {
        _state = FileState::Decrypted;
    }
    if !path.exists() || !path.is_file() {
        _state = FileState::NotFound;
    }
    _state
}

pub fn make_encrypt_path(path: String) -> String {
    format!("{}.{}", path, XPM_EXTENSION)
}

pub fn make_decrypt_path(path: String) -> String{
    let path_split = path
        .split(".")
        .collect::<Vec<&str>>();

    path_split[..path_split.len()-1]
        .join(".")
}