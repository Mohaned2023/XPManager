use colored::Colorize;

pub mod password_manager;

pub fn info(message: &str, delay_ms: i64) {
    println!(
        "[{}] {} - {}",
        "INFO".green(),
        message,
        (delay_ms.to_string()+"ms").green()
    )
}

pub fn error(message: &str) {
    eprintln!(
        "[{}] {}",
        "ERROR".red(),
        message.red()
    )
}