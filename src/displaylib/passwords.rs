use super::Colorize;

pub fn display_one(password: String) {
    println!(
        "{} {}",
        "Password:".blue(),
        password.green()
    )
}