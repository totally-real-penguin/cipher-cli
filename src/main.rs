use std::env;
use std::fs::read_to_string;

mod encrypt;

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Need atleast two arguments. Should be used like this:\n\ncipher-cli /path/to/file [encrypt/decrypt] cipherType")
    }
    let file_path = &args[1];
    let cipher: &String;
    let mut encrypt: bool = true;
    let cipher_text = read_to_string(file_path)
        .expect("Unable to read file");


    match args[2].to_lowercase().as_str() {
        "-d" | "--decode" | "--decrypt" => {
            encrypt = false;
            cipher = &args[3];
        },
        "-e" | "--encode" | "--encrypt" => {
            cipher = &args[3];
        },
        _ => cipher = &args[2]
    };

    match cipher.as_str() {
        "caesar" => encrypt::caesar(cipher_text,encrypt),
        "affine" => encrypt::affine(cipher_text,encrypt),
        "vigenere" => encrypt::vigenere(cipher_text,encrypt),
        _ => panic!("{} is not a valid argument", cipher)
    };
}
