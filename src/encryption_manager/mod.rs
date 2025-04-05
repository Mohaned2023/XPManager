pub mod encrypt_file;
pub mod decrypt_file;

use clap::ArgMatches;
use fernet::Fernet;
use std::io::{Read, Write};