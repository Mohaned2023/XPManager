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