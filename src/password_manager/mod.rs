pub mod generate;
pub mod save;
pub mod find;
pub mod show;
pub mod update;
pub mod count;
pub mod delete;
pub mod encrypt;
pub mod decrypt;

use clap::ArgMatches;

pub struct PMDatabaseEncrption {
    en_path: String,
    de_path: String,
    key: String
}

impl PMDatabaseEncrption {
    pub fn new() -> PMDatabaseEncrption {
        PMDatabaseEncrption { 
            en_path: crate::filelib::pm::get_encrypted_db_path()
                .to_str()
                .unwrap()
                .to_string(),
            de_path: crate::filelib::pm::get_decrypted_db_path()
                .to_str()
                .unwrap()
                .to_string(),
            key: "".to_owned()
        }
    }

    pub fn set_key(&mut self, key: Option<String>) {
        if key == None {
            self.key = crate::utilities::input("Enter the key: ");
        } else {
            self.key = key.unwrap();
        }
    }

    pub fn decrypt(&mut self) {
        self.set_key(None);
        crate::encryption_manager::decrypt_file::decrypt(
            self.en_path.clone(),
            self.key.clone()
        );
        crate::filelib::wipe_delete(self.en_path.clone());
    }

    pub fn encrypt(&self) {
        crate::encryption_manager::encrypt_file::encrypt(
            self.de_path.clone(), 
            self.key.clone()
        );
        crate::filelib::wipe_delete(self.de_path.clone());
    }
}