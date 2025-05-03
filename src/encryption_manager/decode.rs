use super::ArgMatches;
use crate::{
    displaylib, 
    loglib, 
    utilities,
    errorlib,
};

pub fn xpmv1(string: String, n: u16) -> String {
    const XPM_KEY_MARK: &str = "%$%";
    string
        .split(XPM_KEY_MARK)
        .map(|s| {
            let letters = u32::from_str_radix(
                &format!("{}", s.replace("0x", "")),
                16 
            ).unwrap() / n as u32;
            char::from_u32(letters).unwrap()
        })
        .collect::<String>()
}

pub fn hex(string: String) -> String {
    string
        .split(" ")
        .map(|i| {
            char::from_u32(
                u32::from_str_radix(i, 16).unwrap()
            ).unwrap()
        })
        .collect::<String>()
}

pub fn bin(string: String) -> String {
    string
        .split(" ")
        .map(|i| {
            char::from_u32(
                u32::from_str_radix(i, 2).unwrap()
            ).unwrap()
        })
        .collect::<String>()
}

pub fn main(command: &ArgMatches) {
    let mut logger = loglib::Logger::new("decode");
    let string = utilities::input("Enter the string: ");
    logger.start();
    let mut _encoded_date = String::new();
    if *command.get_one::<bool>("xpmv1").unwrap_or(&false) {
        let constant_str = utilities::input("Enter the constant number: ");
        logger.start();
        if let Ok(constant) = constant_str.parse::<u16>() {
            if constant > 9999u16 || constant < 1000u16 {
                logger.error(
                    "constant must be in range (1000 <= x <= 9999)!",
                    errorlib::ExitErrorCode::Input
                )
            }
            _encoded_date = xpmv1(string.clone(), constant);
        } else {
            logger.error(
                "constant must be string!",
                errorlib::ExitErrorCode::Input
            )
        }
    } else if *command.get_one::<bool>("bin").unwrap_or(&false) {
        _encoded_date = bin(string.clone());
    } else {
        _encoded_date = hex(string.clone());
    };
    displaylib::encode::display(&_encoded_date);
    logger.info("string decoded successfully.");
}