
use super::{
    decrypt_file, 
    ArgMatches,
    PathBuf
};
use crate::{
    displaylib, 
    filelib, 
    loglib, 
    utilities,
    dblib
};

pub fn main(command: &ArgMatches) {
    let mut logger = loglib::Logger::new("decrypt-dir");
    if let Some(path) = command.get_one::<String>("PATH") {
        let mut files_paths: Vec<PathBuf> = vec![];
        let is_xpmv1 = *command.get_one::<bool>("xpmv1").unwrap_or(&false);
        let is_delete = *command.get_one::<bool>("delete").unwrap_or(&false);
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
            if is_xpmv1 {
                decrypt_file::xpmv1_decryption(
                    file_path_string.clone(),
                    key.clone()
                );
            } else {
                decrypt_file::decrypt(
                    file_path_string.clone(),
                    key.clone()
                );
            }
            logger.info(
                &format!("decrypted '{}'.", file.display())
            );
            dblib::log::register(
                &format!("file '{}' decrypted", path.clone())
            );
            if is_delete {
                filelib::wipe_delete(file_path_string.clone());
                logger.info("file was wiped successfully.");
                dblib::log::register(
                    &format!("file '{}' wiped", path.clone())
                );
            }
        }
        logger.info("directory decrypted successfully.");
        displaylib::key::display(key);
    }
}