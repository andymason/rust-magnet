use std::io;

fn validate_hash(hash: &str) {
    let length = hash.len();

    if length != 40 {
        panic!("Hash must be 40 characters long, got {}", length)
    } else if !hash.chars().all(|c| c.is_ascii_hexdigit()) {
        panic!("Hash must be all hex digits, got {}", &hash);
    }
}

fn main() {
    println!("Enter a hash");

    let mut hash: String = String::new();

    io::stdin()
        .read_line(&mut hash)
        .expect("Failed to read input");

    let hash = hash.trim();

    validate_hash(hash);

    println!("Hash is {}", hash);
}
