fn main() {
    println!("Hello, world!");

    let s = "Hello, world!";

    let word = first_word(&s);

    println!("The first word is: {}", word);
}


fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for(i,&item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]
}
