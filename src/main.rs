use std::env;
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
    // Get hash from first stdin argument or ask user for  it
    let hash: String = match env::args().nth(1) {
        Some(h) => h,
        None => {
            println!("Enter a hash");
            let mut hash: String = String::new();

            io::stdin()
                .read_line(&mut hash)
                .expect("Failed to read input");

            hash
        }
    };

    let hash = hash.trim();

    validate_hash(hash);

    println!("Hash is {}", hash);
}
