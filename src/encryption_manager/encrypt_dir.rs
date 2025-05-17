
use colored::Colorize;

use super::{
    ArgMatches,
    PathBuf,
    encrypt_file,
    Fernet
};
use crate::{
    displaylib, 
    filelib, 
    loglib, 
    utilities,
    dblib
};

pub fn main(command: &ArgMatches) {
    let mut logger = loglib::Logger::new("encrypt-dir");
    let path = command.get_one::<String>("PATH").unwrap();
    let is_delete = *command.get_one::<bool>("delete").unwrap_or(&false);
    let mut files_paths: Vec<PathBuf> = vec![];
    let key = if *command.get_one::<bool>("key")
        .unwrap_or(&false) {
            let _key = utilities::input("Enter your key: ");
            logger.start();
            _key
    } else {
        Fernet::generate_key()
    };
    filelib::dir_files_tree(
        PathBuf::new().join(path), 
        &mut files_paths
    );
    logger.info("directory listed successfully.");
    if is_delete {
        logger.warning(
            &format!(
                "you are about to encrypt and {} all files in this directory '{}'",
                "delete".red(),
                path.clone()
            )
        );
    } else {
        logger.warning(
            &format!(
                "you are about to encrypt all files in this directory '{}'",
                path.clone()
            )
        );
    }
    utilities::confirm();
    logger.start();
    let log_db_path = filelib::log::get_log_db_path();
    for file in files_paths {
        let file_path_string = file.to_str().unwrap().to_owned();
        if filelib::get_file_state(
            file_path_string.clone()
        ) == filelib::FileState::Encrypted {
            logger.warning(
                &format!("file already encrypted '{}'?", file_path_string)
            );
            continue;
        }
        encrypt_file::encrypt(
            file_path_string.clone(),
            key.clone()
        );
        if is_delete {
            filelib::wipe_delete(file_path_string.clone());
            logger.info(
                &format!("wiped '{}'.", file.display())
            );
        }
        dblib::log::register(
            &format!("encrypted '{}'.", file.display()), 
            log_db_path.clone()
        );
        logger.info(
            &format!("encrypted '{}'.", file.display())
        );
    }
    logger.info("directory encrypted successfully.");
    displaylib::key::display(key);
    dblib::log::register(
        &format!("directory '{}' encrypted and wiped", path),
        log_db_path
    );
}