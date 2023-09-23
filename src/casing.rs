use rand::Rng;

pub fn randomize(text: &mut String) -> String {
    let mut rng = rand::thread_rng();
    let bytes = unsafe { text.as_bytes_mut() };

    for byte in bytes.iter_mut() {
        if rng.gen_bool(0.5) {
            if byte.is_ascii_lowercase() {
                *byte = byte.to_ascii_uppercase();
            }
        } else {
            if byte.is_ascii_uppercase() {
                *byte = byte.to_ascii_lowercase();
            }
        }
    }

    text.clone()
}

pub fn alternate(text: &mut String) -> String {
    let bytes = unsafe { text.as_bytes_mut() };

    for i in 1..bytes.len() {
        let (prev, cur) = (bytes[i - 1], &mut bytes[i]);

        if !cur.is_ascii_alphabetic() {
            continue;
        }
        if prev.is_ascii_lowercase() {
            *cur = cur.to_ascii_uppercase();
        } else if prev.is_ascii_uppercase() {
            *cur = cur.to_ascii_lowercase();
        }
    }

    text.clone()
}

