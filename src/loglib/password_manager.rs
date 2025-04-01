use super::Colorize;

pub fn password(password: String) {
    println!(
        "{} {}",
        "Password:".blue(),
        password.green()
    )
}