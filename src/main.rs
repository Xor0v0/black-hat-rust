use sha1::Digest;
use std::{
    env, error,
    fs::File,
    io::{BufRead, BufReader},
    rc::Rc,
};

const SHA1_HASH_LENGTH: usize = 40;

fn main() -> Result<(), Box<dyn error::Error>> {
    let args = env::args().collect::<Vec<String>>();

    if args.len() != 3 {
        println!("Usage: {} <hash> <wordlist>", args[0]);
        return Ok(());
    }

    let hash_to_crack = args[1].trim();
    if hash_to_crack.len() != SHA1_HASH_LENGTH {
        return Err("Invalid hash length".into());
    }

    let wordlist = File::open(&args[2])?;
    let reader = BufReader::new(wordlist);

    for line in reader.lines() {
        let word = line?;
        let hash = hex::encode(sha1::Sha1::digest(word.trim().as_bytes()));
        if hash == hash_to_crack {
            println!("Found: {}", word);
            return Ok(());
        }
    }
    println!("Pre-image not found.");

    Ok(())
}
