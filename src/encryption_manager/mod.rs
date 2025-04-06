pub mod encrypt_file;
pub mod decrypt_file;
pub mod encrypt_dir;
pub mod decrypt_dir;
pub mod encode;

use clap::ArgMatches;
use fernet::Fernet;
use std::io::{Read, Write};
use std::path::PathBuf;