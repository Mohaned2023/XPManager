use super::Colorize;

pub fn display(new: &str) {
    println!(
        "{}:\n{}\n",
        "The encode".green(),
        new.green()
    )
}