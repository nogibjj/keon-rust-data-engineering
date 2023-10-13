/*

To run:

cargo run --  --message "Off to the bunker. Every person for themselves" --encrypt --shift 10

To decrypt:

cargo run --  --message "Ypp dy dro lexuob. Ofobi zobcyx pyb drowcovfoc" --decrypt --shift 10

*/
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
    
