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
        .chars()
        .map(|i| format!("0x{:x}", i as u32 * n as u32))  
        .collect::<Vec<String>>()
        .join(XPM_KEY_MARK)
}

pub fn hex(string: String) -> String {
    string
        .chars()
        .map(|i| format!("{:X}", i as u32) )
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn hex_hash(string: String) -> String {
    let mut hash = String::new();
    let mut num : u32 = 0;
    for i in string.chars() {
        if i == ' ' {
            hash += &format!(" {:X}", num);
            num = 0;
            continue;
        }
        num += i as u32;
    }
    if num != 0 {
        hash += &format!(" {:X}", num);
    }
    hash.trim().to_owned()
}

pub fn bin(string: String) -> String {
    string
        .chars()
        .map(|i| format!("{:b}", i as u32) )
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn main(command: &ArgMatches) {
    let mut logger = loglib::Logger::new("encode");
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
    } else if *command.get_one::<bool>("hex-hash").unwrap_or(&false) {
        _encoded_date = hex_hash(string.clone());
    } else {
        _encoded_date = hex(string.clone());
    };
    displaylib::encode::display(&_encoded_date);
    logger.info("string encoded successfully.");
}