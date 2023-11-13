use rand::Rng;

pub fn randomize(text: &str) -> String {
    let mut rng = rand::thread_rng();
    text.chars()
        .map(|c| {
            if rng.gen_bool(0.5) {
                c.to_ascii_uppercase()
            } else {
                c.to_ascii_lowercase()
            }
        })
        .collect()
}

pub fn alternate(text: &str) -> String {
    text.chars()
        .enumerate()
        .map(|(i, c)| {
            if i == 0 || !c.is_ascii_alphabetic() {
                c
            } else if text.chars().nth(i - 1).unwrap().is_ascii_lowercase() {
                c.to_ascii_uppercase()
            } else {
                c.to_ascii_lowercase()
            }
        })
        .collect()
}
