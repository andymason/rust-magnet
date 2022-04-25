use std::env;
use std::io;
use urlencoding::encode;

fn validate_hash(hash: &str) {
    let length = hash.len();

    if length != 40 {
        panic!("Hash must be 40 characters long, got {}", length)
    } else if !hash.chars().all(|c| c.is_ascii_hexdigit()) {
        panic!("Hash must be all hex digits, got {}", &hash);
    }
}

fn generate_magnet_link(hash: &str) -> String {
    let sample = "
    https://one.com
    https://two.com
    https://three.com
    https://four.com
    https://five.com
    ";

    let trackers = sample
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.trim())
        .map(|l| format!("&tr={}", l));

    let trackers = trackers.collect::<String>();

    format!("magnet:?xt=urn:btih:{}{}", hash, trackers)
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

    validate_hash(&hash);

    println!("Hash is {}", &hash);

    let link = generate_magnet_link(&hash);
    let link = encode(&link).into_owned();
    print!("Here's the link \n{}", link);
}
