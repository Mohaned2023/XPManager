pub mod encrypt_file;

use clap::ArgMatches;
use fernet::Fernet;
use std::io::{Read, Write};