use aes::Aes128;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use std::fs::File;
use std::io::{Read, Write};

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

fn encrypt_file(key: &[u8], iv: &[u8], input_path: &str, output_path: &str) -> std::io::Result<()> {
    // Open the input file
    let mut input_file = File::open(input_path)?;

    // Read the contents of the input file
    let mut input_data = Vec::new();
    input_file.read_to_end(&mut input_data)?;

    // Create the encryption cipher
    let cipher = Aes128Cbc::new_var(key, iv)?;

    // Encrypt the input data
    let ciphertext = cipher.encrypt_vec(&input_data);

    // Open the output file
    let mut output_file = File::create(output_path)?;

    // Write the encrypted data to the output file
    output_file.write_all(&ciphertext)?;

    Ok(())
}

fn main() -> std::io::Result<()> {
    // Define the key and IV (initialization vector)
    let key = b"0123456789abcdef";
    let iv = b"fedcba9876543210";

    // Encrypt the input file and save the encrypted data to the output file
    encrypt_file(key, iv, "input.txt", "output.bin")?;

    Ok(())
}
