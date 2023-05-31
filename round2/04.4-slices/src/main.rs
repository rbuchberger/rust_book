fn main() {
    hello_slices();

    // String slices are good for passing strings around and referencing them.
    let s = String::from("hello slices");
    let word = first_word(&s);
    println!("first word: {}", word);

    // They keep us safe; it keeps interdependent bits of information
    // synchronized.
    let mut s = String::from("change later");
    let word = first_word(&s);
    // If you switch the order of these two lines, it doesn't compile. That's
    // good, because if we clear out the string then the first word is no longer
    // correct.
    println!("first word: {}", word);
    s.clear();
}

fn hello_slices() {
    // Slices are non-owning pointers. They reference a contiguous sequence of
    // elements in a collection rather than the whole collection.
    let s = String::from("hello world");
    let hello = &s[..5];
    let world = &s[6..];

    let s2 = &s;

    // All the above variables path the same data on the heap.
    println!("{} {}, {}, {}", hello, world, s, s2);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
