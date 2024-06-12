use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::{self, Write, Read};
use std::path::Path;
use rpassword::read_password;
use serde::{Deserialize, Serialize};
use serde_json;
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};

const PASSWORD_FILE: &str = "passwords.enc";
const NONCE: &[u8; 12] = b"unique nonce"; // This nonce should be unique per file

#[derive(Serialize, Deserialize)]
struct PasswordManager {
    passwords: HashMap<String, String>,
}

impl PasswordManager {
    fn new() -> Self {
        PasswordManager {
            passwords: HashMap::new(),
        }
    }

    fn add_password(&mut self, name: String, password: String, master_password: &str) {
        let extended_key = extend_key(master_password, 32);
        let encrypted_password = encrypt_data(password, &extended_key);
        self.passwords.insert(name, encrypted_password);
        self.save_passwords(master_password);
    }

    fn get_password(&self, name: &str, master_password: &str) {
        let extended_key = extend_key(master_password, 32);
        if let Some(encrypted_password) = self.passwords.get(name) {
            match decrypt_data(encrypted_password.clone(), &extended_key) {
                Ok(password) => println!("Mot de passe pour {}: {}", name, password),
                Err(_) => println!("Erreur de déchiffrement du mot de passe."),
            }
        } else {
            println!("Nom non trouvé");
        }
    }

    fn list_password_names(&self) {
        println!("Noms associés aux mots de passe enregistrés :");
        for name in self.passwords.keys() {
            println!("{}", name);
        }
    }

    fn save_passwords(&self, master_password: &str) {
        let extended_key = extend_key(master_password, 32);
        let serialized = serde_json::to_string(&self).unwrap();
        let encrypted_data = encrypt_data(serialized, &extended_key);
        let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(PASSWORD_FILE).unwrap();
        file.write_all(encrypted_data.as_bytes()).unwrap();
    }

    fn load_passwords(master_password: &str) -> Self {
        let extended_key = extend_key(master_password, 32);
        if Path::new(PASSWORD_FILE).exists() {
            let mut file = fs::File::open(PASSWORD_FILE).unwrap();
            let mut encrypted_data = String::new();
            file.read_to_string(&mut encrypted_data).unwrap();
            match decrypt_data(encrypted_data, &extended_key) {
                Ok(decrypted_data) => {
                    serde_json::from_str(&decrypted_data).unwrap()
                },
                Err(_) => {
                    println!("Erreur : Mot de passe maître incorrect.");
                    std::process::exit(1);
                },
            }
        } else {
            PasswordManager::new()
        }
    }
}

fn encrypt_data(data: String, key: &str) -> String {
    let key = Key::from_slice(key.as_bytes());
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(NONCE);
    let ciphertext = cipher.encrypt(nonce, data.as_ref()).expect("encryption failure!");
    base64::encode(&ciphertext)
}

fn decrypt_data(data: String, key: &str) -> Result<String, &'static str> {
    let key = Key::from_slice(key.as_bytes());
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(NONCE);
    let ciphertext = base64::decode(&data).map_err(|_| "decoding failure")?;
    let plaintext = cipher.decrypt(nonce, ciphertext.as_ref()).map_err(|_| "decryption failure")?;
    String::from_utf8(plaintext).map_err(|_| "utf-8 conversion failure")
}

fn extend_key(key: &str, length: usize) -> String {
    let mut extended_key = key.to_string();
    while extended_key.len() < length {
        extended_key.push_str(key);
    }
    extended_key.truncate(length);
    extended_key
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage : <command> <args>");
        println!("Commands:");
        println!("  add <name>");
        println!("  get <name>");
        println!("  list");
        return;
    }

    let command = &args[1];
    let master_password = prompt_secure("Entrez le mot de passe maître : ");
    let mut password_manager = PasswordManager::load_passwords(&master_password);

    match command.as_str() {
        "add" => {
            if args.len() != 3 {
                println!("Usage : add <name>");
                return;
            }
            let name = &args[2];
            let password = prompt_secure("Entrez le mot de passe à ajouter : ");
            password_manager.add_password(name.to_string(), password, &master_password);
        },
        "get" => {
            if args.len() != 3 {
                println!("Usage : get <name>");
                return;
            }
            let name = &args[2];
            password_manager.get_password(name, &master_password);
        },
        "list" => {
            password_manager.list_password_names();
        },
        _ => {
            println!("Commande non reconnue. Utilisez 'add', 'get' ou 'list'.");
        },
    }
}

fn prompt_secure(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    read_password().unwrap()
}
