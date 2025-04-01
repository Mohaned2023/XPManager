use chrono::Local;
use colored::Colorize;

pub mod password_manager;

pub struct Logger {
    name: String,
    start_time_ms: i64
}

impl Logger {
    pub fn new( name: &str ) -> Logger {
        Logger {
            name: name.to_owned(),
            start_time_ms:  Local::now().timestamp_millis()
        }
    }

    pub fn end(&self) -> String {
        format!(
            "{}ms",
            Local::now().timestamp_millis() - self.start_time_ms
        )
    }

    pub fn info(&self, message: &str) {
        println!(
            "[{}] - [{}] {} - {}",
            "INFO".green(),
            self.name.green(),
            message,
            self.end().green()
        )
    }

    pub fn error(&self, message: &str) {
        eprintln!(
            "[{}] - [{}] {}",
            "ERROR".red(),
            self.name.red(),
            message.red()
        )
    }
}