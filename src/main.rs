use clap::Command;

mod password_manager;
mod commands;

fn main() {
    let matches = Command::new("xpm")
        .about("Password, file, and backup manager. All in one tool XPManger ✔")
        .version("2.0")
        .author("Mohaned Sherhan")
        .subcommands(commands::commands())
        .get_matches();
}
