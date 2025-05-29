use std::fs::File;
use std::{fs, io};
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use std::process::exit;
use random_str::get_string;
use rpassword::prompt_password;
use sha2::{Digest, Sha256};

const ARREST: &str = "ar";
const RELEASE: &str = "rl";
const PASSWORD_LENGTH: usize = 64;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <arrest ({}) or release ({})> <file>", args[0], ARREST, RELEASE);
        exit(1);
    }
    if args[1] != ARREST && args[1] != RELEASE {
        println!("Usage: {} <arrest ({}) or release ({})> <file>", args[0], ARREST, RELEASE);
        exit(1);
    }
    
    let file = &args[2];
    if !Path::new(file).exists() {
        println!("File {} does not exist", file);
        exit(1);
    }
    
    if args[1] == ARREST {
        println!("Arresting {}", file);
        arrest(file);
    } else {
        let key = prompt_password("Enter key: ").expect("Failed to read password");
        println!("Releasing {}", file);
        release(file, key.as_str());
    }
}

fn process_file(file: &str, password: &str, encrypt: bool) {
    let tmp_file = format!("{}.tmp", file);
    let mut c = encryptfile::Config::new();
    c.input_stream(encryptfile::InputStream::File(file.to_owned()))
        .output_stream(encryptfile::OutputStream::File(tmp_file.clone()))
        .add_output_option(encryptfile::OutputOption::AllowOverwrite)
        .initialization_vector(encryptfile::InitializationVector::GenerateFromRng)
        .password(encryptfile::PasswordType::Text(
            password.to_owned(),
            encryptfile::scrypt_defaults(),
        ));

    if encrypt {
        c.encrypt();
    } else {
        c.decrypt();
    }

    encryptfile::process(&c).unwrap_or_else(|e| 
        { panic!("Error {}crypting: {:?}", if encrypt { "en" } else { "de" }, e) });
    fs::rename(&tmp_file, file).expect("Failed to overwrite original file");
}


fn arrest(file: &str) {
    let sha256 = sha256sum(file);
    let password = get_string(PASSWORD_LENGTH, true, true, true, false);
    process_file(file, &password, true);
    println!("Your key: {}", get_key(&sha256, &password));
}

fn release(file: &str, key: &str) {
    let (sha256, password) = unpack_key(key);
    process_file(file, &password, false);
    if sha256sum(file) != sha256 {
        eprintln!("Checksum mismatch");
    }
}

/// Get the SHA256 sum of a file
fn sha256sum(path: &str) -> String {
    let input = File::open(path).expect("Failed to open file");
    let mut reader = BufReader::new(input);

    let digest = {
        let mut hasher = Sha256::new();
        let mut buffer = [0; 1024];
        loop {
            let count = reader.read(&mut buffer).expect("Failed to read");
            if count == 0 { break }
            hasher.update(&buffer[..count]);
        }
        hasher.finalize()
    };
    format!("{:x}", digest)
}

/// Get key
fn get_key(sha256: &str, password: &str) -> String {
    sha256.chars().zip(password.chars())
        .flat_map(|(a, b)| [a, b])
        .collect()
}

/// Get sha256 and password from a key
fn unpack_key(key: &str) -> (String, String) {
    let mut sha256 = String::new();
    let mut password = String::new();

    for (i, ch) in key.chars().enumerate() {
        if i % 2 == 0 {
            sha256.push(ch);
        } else {
            password.push(ch);
        }
    }

    (sha256, password)
}
