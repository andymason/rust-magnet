use std::env;
use std::io;
use std::process;
use ureq;
use urlencoding::encode;

fn validate_hash(hash: &str) {
    let length = hash.len();

    if length != 40 {
        eprintln!(
            "Hash must be 40 characters long, got length {} - {}",
            length, &hash
        );
        process::exit(1)
    } else if !hash.chars().all(|c| c.is_ascii_hexdigit()) {
        eprintln!("Hash must be all hex digits, got {}", &hash);
        process::exit(1)
    }
}

fn get_trackers() -> String {
    const TRACKER_API_URL: &str = "https://newtrackon.com/api/stable";

    match ureq::get(TRACKER_API_URL).set("e", "dr").call() {
        Ok(body) => body.into_string().unwrap(),
        Err(_) => {
            eprintln!("Failed to get trackers");
            process::exit(1)
        }
    }
}

fn generate_magnet_link(hash: &str, tracker_list: &str) -> String {
    let trackers = tracker_list
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.trim())
        .map(|l| encode(l).into_owned())
        .map(|l| format!("&tr={}", l));

    let trackers = trackers.collect::<String>();

    format!("magnet:?xt=urn:btih:{}{}", hash, trackers)
}

fn main() {
    // Get hash from first stdin argument or ask user for  it
    let hash: String = match env::args().nth(1) {
        Some(h) => h,
        None => {
            println!("Magnet Link Generator v0.1\nEnter a hash:");
            let mut hash: String = String::new();

            io::stdin()
                .read_line(&mut hash)
                .expect("Failed to read input");

            hash
        }
    };

    let hash = hash.trim();

    validate_hash(&hash);

    let trackers = get_trackers();

    let link = generate_magnet_link(&hash, &trackers);

    println!("{}", link);
}
