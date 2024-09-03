use std::fs::File;
use std::io::Read;
use solana_sdk::signature::Keypair;

pub fn read_keypair_from_file(file_path: &str) -> Result<Keypair, Box<dyn std::error::Error>> {
    // Open the file
    let mut file = File::open(file_path)?;
    
    // Read the file's contents into a string
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)?;
    
    // Deserialize the JSON content into a Keypair
    let keypair = Keypair::from_base58_string(&file_content.trim());
    
    Ok(keypair)
}