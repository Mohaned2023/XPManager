
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
    if let Some(path) = command.get_one::<String>("PATH") {
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
        for file in files_paths {
            let file_path_string = file.to_str().unwrap().to_owned();
            encrypt_file::encrypt(
                file_path_string.clone(),
                key.clone()
            );
            filelib::wipe_delete(file_path_string.clone());
            logger.info(
                &format!("encrypted and wiped '{}'.", file.display())
            );
        }
        logger.info("directory encrypted successfully.");
        displaylib::key::display(key);
        dblib::log::register(
            &format!("directory '{}' encrypted and wiped", path)
        );
    }
}