fn main() {}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &char) in bytes.iter().enumerate() {
        if char == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
