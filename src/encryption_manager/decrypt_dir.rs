
use super::{
    decrypt_file, 
    ArgMatches,
    PathBuf
};
use crate::{
    displaylib, 
    filelib, 
    loglib, 
    utilities
};

pub fn main(command: &ArgMatches) {
    let mut logger = loglib::Logger::new("decrypt-dir");
    if let Some(path) = command.get_one::<String>("PATH") {
        let mut files_paths: Vec<PathBuf> = vec![];
        let key = utilities::input("Enter your key: ");
        logger.start();
        filelib::dir_files_tree(
            PathBuf::new().join(path), 
            &mut files_paths
        );
        logger.info("directory listed successfully.");
        for file in files_paths {
            let file_path_string = file.to_str().unwrap().to_owned();
            if filelib::get_file_state(
                file_path_string.clone()
            ) == filelib::FileState::Decrypted {
                logger.warning(
                    &format!("file not encrypted '{}'?", file_path_string)
                );
                continue;
            }
            decrypt_file::decrypt(
                file_path_string.clone(),
                key.clone()
            );
            filelib::wipe_delete(file_path_string.clone());
            logger.info(
                &format!("decrypted and wiped '{}'.", file.display())
            );
        }
        logger.info("directory decrypted successfully.");
        displaylib::key::display(key);
    }
}