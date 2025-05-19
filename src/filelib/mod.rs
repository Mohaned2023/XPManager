pub mod pm;
pub mod log;

use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::{PathBuf, Path};
use std::fs::OpenOptions;
use std::io::{
    BufReader, BufWriter, Read, Seek, SeekFrom, Write
};
use dirs::data_dir;
use rand::Rng;
use serde_json::Value;
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
                    &format!("can NOT create the directory at '{}'!", parent.display()),
                    errorlib::ExitErrorCode::DirCreate
                );
            }
        }
    }
    if let Err(_) = std::fs::File::create(&path) {
        logger.error(
            &format!("can NOT create the file at '{}'!", path.display()),
                errorlib::ExitErrorCode::FileCreate
            );
    }
    logger.info(
        &format!("create file at '{}'", path.display())
    );
}

pub fn delete_file(path: PathBuf) {
    let logger = loglib::Logger::new("delete-file");
    if path.exists() {
        if let Err(_) = std::fs::remove_file(&path) {
            logger.error(
                &format!("can NOT delete the file at '{}'!", path.display()),
                errorlib::ExitErrorCode::FileDelete
            );
        }
    }
}

fn wipe_file(path: String, wipe_type: WipeType) {
    let logger = loglib::Logger::new("wipe-file");
    let path = Path::new(&path);
    if !path.exists() || !path.is_file() {
        logger.error(
            "file NOT found!", 
            errorlib::ExitErrorCode::FileNotFound
        );
    }
    if let Ok(mut file) = OpenOptions::new()
        .write(true) 
        .open(path) {
        if let Ok(metadata) = file.metadata() {
            let len = metadata.len();
            if len == 0 {
                // File len is 0, file is empty,
                // we can not wipe an empty file.
                return;
            }
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
                        errorlib::ExitErrorCode::FileSeek
                    );
                }
                if let Err(_) = file.write_all(&data) {
                    logger.error(
                        "can NOT write to the file!", 
                        errorlib::ExitErrorCode::FileWrite
                    );
                }
                pos += size as u64;
            }
            if let Err(_) = file.flush() {
                logger.error(
                    "can NOT flush the file to the disk!", 
                    errorlib::ExitErrorCode::FileFlush
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

pub fn dir_files_tree(folder_path: PathBuf, files_paths: &mut Vec<PathBuf> ){
    let logger = loglib::Logger::new("dir-files-tree");
    if !folder_path.exists() {
        logger.error(
            "can NOT find the directory!", 
            errorlib::ExitErrorCode::DirNotFound
        );
    }
    if let Ok(paths) = folder_path.read_dir() {
        for p in paths {
            if let Ok(entry) = p {
                if let Ok(file_type) = entry.file_type() {
                    let entry_path = entry.path();
                    if file_type.is_file() {
                        files_paths.push(entry_path);
                    } else if file_type.is_dir() {
                        dir_files_tree(entry_path, files_paths);
                    } else {
                        logger.error(
                            &format!("unsupported directory at '{}'!", entry_path.display()),
                            errorlib::ExitErrorCode::DirUnsupported
                        )
                    }
                } else {
                    logger.error(
                        "can NOT get the file/folder type!", 
                        errorlib::ExitErrorCode::CanNotGetFileOrDirType
                    )
                }
            } else {
                logger.error(
                    "can NOT get the folder entry!", 
                    errorlib::ExitErrorCode::CanNotGetDirData
                )
            }
        }
    } else {
        logger.error(
            "can NOT get the folder data!", 
            errorlib::ExitErrorCode::CanNotGetDirData
        )
    }
}

pub fn copy(file: String, to_file: String) {
    let logger = loglib::Logger::new("copy-file");
    let file_path = PathBuf::new().join(file);
    if !file_path.exists() || !file_path.is_file() {
        logger.error(
            "file NOT found!",
            errorlib::ExitErrorCode::FileNotFound
        )
    }
    let file_stream = std::fs::File::open(file_path).unwrap();
    if let Ok(to_file)= std::fs::File::create(to_file) {
        let mut reader = BufReader::new(file_stream);
        let mut writer = BufWriter::new(to_file);
        let mut buffer = vec![0; 64 * 1024]; // 64KB
        loop {
            let bytes_read = reader.read(&mut buffer).unwrap();
            if bytes_read == 0 {
                break;
            }
            writer.write_all(&buffer[..bytes_read]).unwrap();
        }
        writer.flush().unwrap();
    } else {
        logger.error(
            "directory NOT found!",
            errorlib::ExitErrorCode::DirNotFound
        )
    }
}

pub fn read_json(file: String) -> HashMap<String, String> {
    let logger = loglib::Logger::new("read-json");
    let json_path = PathBuf::new().join(file);
    let mut contents = String::new();
    if let Ok(mut json_file) = std::fs::File::open(json_path) {
        json_file.read_to_string(&mut contents).unwrap();
    }
    if let Ok(json) = serde_json::from_str(&contents) {
        if let Value::Object(map) = json {
            let data: HashMap<String, String> = map.into_iter()
                .filter_map(|(key, value)| {
                    if let Value::String(val) = value {
                        Some((key, val))
                    } else {
                        logger.error(
                            "invalid json file!",
                            errorlib::ExitErrorCode::InvalidJson
                        )
                    }
                }).collect();
            return data;
        }
    }
    logger.error(
        "can not get the json data!", 
        errorlib::ExitErrorCode::CanNotGetJsonObject
    )
}


#[cfg(test)]
mod tests {
    use std::io::Write;

    #[test]
    fn create_file() {
        let temp_dir = super::PathBuf::new()
            .join("./temp/create_file");
        if temp_dir.exists() {
            std::fs::remove_dir_all(&temp_dir)
                .expect("Can NOT remove test temp dir!!");
        }
        let file_path = temp_dir.join("test.txt");
        super::create_file(file_path.clone());
        assert_eq!(file_path.exists(), true, "Can NOT create the test file!!");
        std::fs::remove_dir_all(temp_dir)
            .expect("Can NOT remove test temp dir!!");
    }

    #[test]
    fn delete_file() {
        let file_path = super::PathBuf::new()
            .join("./temp/delete_file/test.txt");
        super::create_file(file_path.clone());
        assert_eq!(file_path.exists(), true, "Can NOT create the test file!!");
        super::delete_file(file_path.clone());
        assert_eq!(file_path.exists(), false, "File NOT deleted!!");
    }

    #[test]
    fn wipe_delete() {
        let file_path = super::PathBuf::new()
            .join("./temp/wipe_delete/test.txt");
        let message = "this is test message!";
        super::create_file(file_path.clone());
        assert_eq!(file_path.exists(), true, "Can NOT create the test file!!");
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .open(&file_path)
            .expect("Can NOT open the test file!!");
        file
            .write_all(message.as_bytes() )
            .expect("Can NOT write all to the test file!!");
        super::wipe_delete(
            file_path
                .to_str()
                .expect("Can NOT parse PathBuf to &str")
                .to_string()
        );
        assert_eq!(file_path.exists(), false, "File NOT wiped and deleted!!");
    }

    #[test]
    fn get_file_state() {
        let temp_dir = super::PathBuf::new()
            .join("./temp/get_file_state");
        if temp_dir.exists() {
            std::fs::remove_dir_all(temp_dir.clone())
                .expect("Can NOT delete the temp tests dir!!");
        }
        let de_file_path = temp_dir.join(
            "test.txt"
        );
        let en_file_path = temp_dir.join(
            "test.txt.x"
        );

        // File not exists
        let mut state = super::get_file_state(
            "./temp/get_file_state/test.txt".to_string()
        ) == super::FileState::NotFound;
        assert_eq!(state, true, "File state NOT match!!");

        // File is decrypted
        super::create_file(de_file_path.clone());
        assert_eq!(de_file_path.exists(), true, "Can NOT create test file!!");
        state = super::get_file_state(
            de_file_path
                .to_str()
                .expect("Can NOT parse PathBuf to &str!!")
                .to_string()
        ) == super::FileState::Decrypted;
        assert_eq!(state, true, "File state NOT match!!");

        // File is encrypted
        super::create_file(en_file_path.clone());
        assert_eq!(de_file_path.exists(), true, "Can NOT create test file!!");
        state = super::get_file_state(
            en_file_path
                .to_str()
                .expect("Can NOT parse PathBuf to &str!!")
                .to_string()
        ) == super::FileState::Encrypted;
        assert_eq!(state, true, "File state NOT match!!");

        // Delete temp files
        std::fs::remove_dir_all(temp_dir)
            .expect("Can NOT delete the temp tests dir!!");
    }

    #[test]
    fn make_encrypt_path() {
        let file = super::make_encrypt_path(
            "./temp/make_encrypt_path/test.txt".to_string()
        );
        assert_eq!(
            file, 
            "./temp/make_encrypt_path/test.txt.x",
            "Can NOT create encryption path!!"
        );
    }

    #[test]
    fn make_decrypt_path() {
        let file = super::make_decrypt_path(
            "./temp/make_decrypt_path/test.txt.x".to_string()
        );
        assert_eq!(
            file, 
            "./temp/make_decrypt_path/test.txt",
            "Can NOT create decryption path!!"
        );
    }

    #[test]
    fn dir_files_tree() {
        let temp_dir = super::PathBuf::new()
            .join("./temp/dir_files_tree");
        let mut files_paths: Vec<super::PathBuf> = vec![];
        let files: [super::PathBuf; 4] = [
            temp_dir.join("test.txt"),
            temp_dir.join("dir/test-0.txt"),
            temp_dir.join("dir/test-1.txt"),
            temp_dir.join("dir/files/test.txt")
        ];
        for file in files.clone() {
            super::create_file(file.clone());
            assert_eq!(file.exists(), true, "Can NOT create the test file!!");
        }
        super::dir_files_tree(temp_dir.clone(), &mut files_paths);
        let mut found: bool = false;
        for file in files {
            for tree in files_paths.as_slice() {
                // replace \ or / with -
                // result will be like: 
                //      before: './temp/dir_files_tree\\dir/test-0.txt'
                //      after : '.-temp-dir_files_tree-dir-test-0.txt'
                if tree
                    .to_str()
                    .expect("Can NOT parse PathBuf to &str")
                    .replace("/", "-")
                    .replace("\\", "-") == file
                    .to_str()
                    .expect("Can NOT parse PathBuf to &str")
                    .replace("/", "-")
                    .replace("\\", "-") {
                    found = true;
                }
            }
            assert_eq!(found, true, "Files tree NOT match!!");
            found = false;
        }
        std::fs::remove_dir_all(temp_dir)
            .expect("Can NOT delete the temp tests dir!!");
    }

    #[test]
    fn copy() {
        let temp_dir = super::PathBuf::new()
            .join("./temp/copy");
        let file = temp_dir.join("src.txt");
        super::create_file(file.clone());
        assert_eq!(file.exists(), true, "Can NOT create the test file!!");
        let to = temp_dir.join("to.txt");
        super::copy(
            file
                .to_str()
                .expect("Can NOT parse PathBuf to &str!!")
                .to_string(), 
            to
                .to_str()
                .expect("Can NOT parse PathBuf to &str!!")
                .to_string()
        );
        assert_eq!(to.exists(), true, "Can NOT copy the test file!!");
        std::fs::remove_dir_all(temp_dir)
            .expect("Can NOT delete the temp tests dir!!");
    }
}