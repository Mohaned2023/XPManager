use super::Colorize;

pub fn display(key: String) {
    println!(
        "\n{} {}\n",
        "Your Key:".blue(),
        key.green()
    );
}