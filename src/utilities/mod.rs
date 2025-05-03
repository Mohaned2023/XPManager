
use crate::{
    errorlib, 
    loglib
};
use colored::Colorize;
use rand::seq::{IndexedRandom, IteratorRandom};

#[derive(PartialEq)]
pub enum PasswordSample {
    Ascii,
    Hex
}
pub fn get_sample(sample: PasswordSample) -> Vec<char> {
    if sample == PasswordSample::Ascii {
        ('a'..='z')
        .chain('A'..='Z')
        .chain('0'..='9')
        .chain([
            '!', '@', '#', '$', '%', '^', '&', '(', ')', '-', '+', '=', '~',
            '[', ']', '{', '}', '/', '|', ':', ';', '?', ',', '.', '<', '>'
        ])
        .collect()
    } else {
        ('0'..='9')
        .chain('A'..='F')
        .collect()
    }
}

pub fn get_ran_string_number() -> String {
    let mut rag = rand::rng();
    // Choose random length between 32 to 72.
    return (32..=72)
        .choose(&mut rag)
        .unwrap()
        .to_string();
}

pub fn input(message: &str) -> String {
    use std::io::Write;
    print!("{}", message);
    std::io::stdout().flush().expect("Flush Error!");
    let mut line: String = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Read line Error!");
    return line.trim().to_owned();
}

pub fn confirm() {
    let mut logger = loglib::Logger::new("confirm");
    logger.warning("This process requires confirmation!");
    let mut rng = rand::rng();
    let sample: Vec<char> = ('1'..='9').collect();
    let mut confirmation = String::new();
    for _ in 0..6 {
        confirmation.push(
            sample.choose(&mut rng)
                .unwrap()
                .clone()
        );
    }
    let value = input(
        &format!("Please enter {} to continue: ", confirmation.green())
    );
    logger.start();
    if value != confirmation {
        logger.error(
            "This process stopped, confirmation error",
            errorlib::ExitErrorCode::ConfirmationNotMatch
        )
    }
    logger.info("confirmation completed successfully.");
}