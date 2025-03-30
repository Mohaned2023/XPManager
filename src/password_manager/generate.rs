use rand::seq::{IndexedRandom, SliceRandom};

pub fn run(length: u16 ) -> String {
    let mut rng = rand::rng();
    let mut ascii: Vec<char> = ('a'..='z')
        .chain('A'..='Z')
        .chain('0'..='9')
        .chain([
            '!', '@', '#', '$', '%', '^', '&', '(', ')', '-', '+', '=', '~',
            '[', ']', '{', '}', '/', '|', ':', ';', '?', ',', '.', '<', '>'
        ])
        .collect();
    ascii.shuffle(&mut rng);
    let mut password: String = String::new();
    for _ in 0..length {
        password.push(
            ascii.choose(&mut rng)
                .unwrap()
                .clone()
        );
    }
    password
}