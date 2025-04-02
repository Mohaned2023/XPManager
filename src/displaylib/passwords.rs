use super::Colorize;
use crate::dblib;

pub fn display_one(password: String) {
    println!(
        "{} {}",
        "Password:".blue(),
        password.green()
    )
}

pub fn display_many(passwords: Vec<dblib::PasswordInfoForm>, string: String) {
    for pass in passwords {
        let mut name = pass.name.clone();
        if let Some(start_string_pos) = pass
            .name
            .to_lowercase()
            .find(&string.to_lowercase()) {
                let end_string_pos = start_string_pos+string.len();
                let slice_string = &name[start_string_pos..end_string_pos];
                name.replace_range(
                    start_string_pos..end_string_pos,
                    &slice_string.red().to_string()
                );
        }
        println!(
            "{} - {}: {}",
            pass.id.to_string().blue(),
            name.blue(),
            pass.password.green()
        )
    }
}