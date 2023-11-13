use rand::Rng;

pub fn randomize(text: &str) -> String {
    let mut rng = rand::thread_rng();
    text.chars()
        .map(|c| {
            if !c.is_alphabetic() || !c.is_ascii() {
                c
            } else if rng.gen_bool(0.5) {
                if c.is_ascii_uppercase() {
                    c
                } else {
                    c.to_ascii_uppercase()
                }
            } else if c.is_ascii_lowercase() {
                c
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
            if !c.is_alphabetic() || !c.is_ascii() {
                c
            } else if i % 2 == 0 {
                if c.is_ascii_uppercase() {
                    c
                } else {
                    c.to_ascii_uppercase()
                }
            } else if c.is_ascii_lowercase() {
                c
            } else {
                c.to_ascii_lowercase()
            }
        })
        .collect()
}
