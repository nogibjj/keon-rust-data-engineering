/*

To run:

cargo run --  --message "Off to the bunker. Every person for themselves" --encrypt --shift 10

To decrypt:

cargo run --  --message "Ypp dy dro lexuob. Ofobi zobcyx pyb drowcovfoc" --decrypt --shift 10

*/


// use caeser_cipher_cli::{decrypt, encrypt};
// use clap::Parser;

// /// CLI tool to encrypt and decrypt messages using the caeser cipher
// #[derive(Parser, Debug)]
// #[command(author, version, about, long_about = None)]
// struct Args {
//     /// Encrypt the message
//     #[arg(short, long)]
//     encrypt: bool,

//     /// decrypt the message
//     #[arg(short, long)]
//     decrypt: bool,

//     /// The message to encrypt or decrypt
//     #[arg(short, long)]
//     message: String,

//     /// The shift to use for the cipher
//     /// Must be between 1 and 25, the default is 3
//     #[arg(short, long, default_value = "3")]
//     shift: u8,
// }

// // run it
// fn main() {
//     let args = Args::parse();
//     if args.encrypt {
//         println!("{}", encrypt(&args.message, args.shift));
//     } else if args.decrypt {
//         println!("{}", decrypt(&args.message, args.shift));
//     } else {
//         println!("Please specify either --encrypt or --decrypt");
//     }
// }


pub fn encrypt(text: &str, shift: u8) -> String {
    let mut result = String::new();
    for c in text.chars() {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            let offset = (c as u8 - base + shift) % 26;
            result.push((base + offset) as char);
        } else {
            result.push(c);
        }
    }
    result
}

pub fn decrypt(text: &str, shift: u8) -> String {
    encrypt(text, 26 - shift)
}

fn main() {
    let plaintext = "The quick brown fox jumps over the lazy dog.";
    let shift = 5;

    let ciphertext = encrypt(plaintext, shift);
    let decrypted_text = decrypt(&ciphertext, shift);

    println!("Plaintext: {}", plaintext);
    println!("Shift: {}", shift);
    println!("Ciphertext: {}", ciphertext);
    println!("Decrypted text: {}", decrypted_text);
}
