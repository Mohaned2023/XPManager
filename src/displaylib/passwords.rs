use super::Colorize;
use crate::dblib;
use tabled::{
    settings::{
        location::ByColumnName, 
        object::{Columns, Rows}, 
        Alignment, 
        Modify, 
        Remove, 
        Style, 
        Width
    }, 
    Table
};
use crossterm::terminal;

pub fn display_one(password: String) {
    println!(
        "\n{} {}\n",
        "Password:".blue(),
        password.green()
    )
}

pub fn display_many(passwords: Vec<dblib::PasswordInfoForm>, string: String) {
    println!();
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
            "{} - {} - {} - {}: {}",
            pass.id.to_string().blue(),
            pass.create_at.blue(),
            pass.update_at.blue(),
            name.blue(),
            pass.password.green()
        )
    }
    println!();
}

pub fn display_as_table(passwords: Vec<dblib::PasswordInfoForm>) {
    let (width, _) = terminal::size().unwrap_or((80, 0));
    let max_col_width = (width as f32 * 0.7 ) as usize;
    println!(
        "{}",
        Table::new(passwords)
            .with(Style::rounded())
            .with( // First Row: Set the alignment to center.
                Modify::new(
                    Rows::single(0)
                ).with(Alignment::center())
            )
            .with( // Name Column: Set the max based on terminal width.
                Modify::new(
                    Columns::single(1)
                ).with(
                    Width::wrap(max_col_width)
                )
            )
            .with( // Remove The Password Column.
                Remove::column(
                    ByColumnName::new("password")
                )
            )
    );
}