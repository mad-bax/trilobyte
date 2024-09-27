use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
    process::exit,
};

use rand::random;

const TRILOBYTE_VERSION: &str = env!("CARGO_PKG_VERSION");
const READ_SIZE: usize = 1024;

fn generation_error(code: &str) {
    eprintln!("Generation error code: {}", code);
}

fn encryption_error(code: &str) {
    eprintln!("Encryption error code: {}", code);
}

fn decryption_error(code: &str) {
    eprintln!("Decryption error code: {}", code);
}

fn otp_generate(size: usize) -> Vec<u8> {
    (0..size).map(|_| random()).collect()
}

fn otp_key_create(pairs: Vec<(usize, String)>) {
    let local_path = Path::new(".");

    for (size, filename) in pairs {
        let file_buff = PathBuf::from(&filename);

        let file_stem = file_buff.file_stem().unwrap();
        let file_path = match file_buff.parent() {
            Some(v) => {
                if v.to_str().unwrap() == "" {
                    local_path
                } else {
                    v
                }
            }
            None => local_path,
        };

        let otp = otp_generate(size);
        let mut cef_file = match File::create(format!(
            "{}/{}.cef",
            file_path.to_str().unwrap(),
            file_stem.to_str().unwrap()
        )) {
            Ok(v) => v,
            Err(_) => {
                encryption_error("Lion");
                continue;
            }
        };
        cef_file.write_all(&otp).unwrap();
    }
}

fn otp_encrypt_static(files: Vec<(String, String)>) {
    let local_path = Path::new(".");

    for (data_file, key_file) in files {
        let file_buff = PathBuf::from(&data_file);

        if !file_buff.exists() {
            encryption_error("Moose");
            continue;
        }

        let file_stem = file_buff.file_stem().unwrap();
        let file_ext = file_buff.extension().unwrap();
        let file_path = match file_buff.parent() {
            Some(v) => {
                if v.to_str().unwrap() == "" {
                    local_path
                } else {
                    v
                }
            }
            None => local_path,
        };

        let mut final_data: Vec<u8> = vec![];

        let mut handle = match File::open(data_file.clone()) {
            Ok(v) => v,
            Err(_) => {
                encryption_error("Hawk");
                continue;
            }
        };
        let mut data = [0; READ_SIZE];

        loop {
            match handle.read(&mut data) {
                Ok(0) => break,
                Ok(n) => {
                    for val in data.iter().take(n) {
                        final_data.push(*val);
                    }

                    if n < READ_SIZE {
                        break;
                    }
                }
                Err(_) => {
                    encryption_error("Squirrel");
                    continue;
                }
            }
        }

        let mut handle = match File::open(key_file.clone()) {
            Ok(v) => v,
            Err(_) => {
                encryption_error("Bee");
                continue;
            }
        };

        let mut key_data: Vec<u8> = vec![];

        loop {
            match handle.read(&mut data) {
                Ok(0) => break,
                Ok(n) => {
                    for val in data.iter().take(n) {
                        key_data.push(*val)
                    }

                    if n < READ_SIZE {
                        break;
                    }
                }
                Err(_) => {
                    decryption_error("Cat");
                    continue;
                }
            }
        }

        if final_data.len() > key_data.len() {
            encryption_error("Eagle");
            continue;
        }

        for i in 0..final_data.len() {
            final_data[i] ^= key_data[i];
        }

        let mut csd_file = match File::create(format!(
            "{}/{}.{}.csd",
            file_path.to_str().unwrap(),
            file_stem.to_str().unwrap(),
            file_ext.to_str().unwrap()
        )) {
            Ok(v) => v,
            Err(_) => {
                encryption_error("Crab");
                continue;
            }
        };
        csd_file.write_all(&final_data).unwrap();
    }
}

fn otp_encrypt(files: Vec<String>) {
    let local_path = Path::new(".");

    for f in files {
        let file_buff = PathBuf::from(&f);

        if !file_buff.exists() {
            encryption_error("Owl");
            continue;
        }

        let file_stem = file_buff.file_stem().unwrap();
        let file_ext = file_buff.extension().unwrap();
        let file_path = match file_buff.parent() {
            Some(v) => {
                if v.to_str().unwrap() == "" {
                    local_path
                } else {
                    v
                }
            }
            None => local_path,
        };

        let mut final_data: Vec<u8> = vec![];

        let mut handle = match File::open(f.clone()) {
            Ok(v) => v,
            Err(_) => {
                encryption_error("Deer");
                continue;
            }
        };
        let mut data = [0; READ_SIZE];

        loop {
            match handle.read(&mut data) {
                Ok(0) => break,
                Ok(n) => {
                    for val in data.iter().take(n) {
                        final_data.push(*val);
                    }

                    if n < READ_SIZE {
                        break;
                    }
                }
                Err(_) => {
                    encryption_error("Badger");
                    continue;
                }
            }
        }

        let otp: Vec<u8> = otp_generate(final_data.len());

        for i in 0..final_data.len() {
            final_data[i] ^= otp[i];
        }

        let mut cef_file = match File::create(format!(
            "{}/{}.cef",
            file_path.to_str().unwrap(),
            file_stem.to_str().unwrap()
        )) {
            Ok(v) => v,
            Err(_) => {
                encryption_error("Bear");
                continue;
            }
        };
        cef_file.write_all(&otp).unwrap();

        let mut csd_file = match File::create(format!(
            "{}/{}.{}.csd",
            file_path.to_str().unwrap(),
            file_stem.to_str().unwrap(),
            file_ext.to_str().unwrap()
        )) {
            Ok(v) => v,
            Err(_) => {
                encryption_error("Tuna");
                continue;
            }
        };
        csd_file.write_all(&final_data).unwrap();
    }
}

fn otp_decrypt(pairs: Vec<(String, String)>) {
    let local_path = Path::new(".");

    for (data_file, key_file) in pairs {
        let data_file_buff = PathBuf::from(&data_file);

        if !data_file_buff.exists() {
            decryption_error("Snake");
            continue;
        }

        let data_file_stem = data_file_buff.file_stem().unwrap();
        let data_file_path = match data_file_buff.parent() {
            Some(v) => {
                if v.to_str().unwrap() == "" {
                    local_path
                } else {
                    v
                }
            }
            None => local_path,
        };

        let key_file_buff = PathBuf::from(&key_file);

        if !key_file_buff.exists() {
            decryption_error("Orca");
            continue;
        }

        let mut final_data: Vec<u8> = vec![];
        let mut key_data: Vec<u8> = vec![];

        let mut handle = match File::open(data_file.clone()) {
            Ok(v) => v,
            Err(_) => {
                decryption_error("Monkey");
                continue;
            }
        };
        let mut data = [0; READ_SIZE];

        loop {
            match handle.read(&mut data) {
                Ok(0) => break,
                Ok(n) => {
                    for val in data.iter().take(n) {
                        final_data.push(*val);
                    }

                    if n < READ_SIZE {
                        break;
                    }
                }
                Err(_) => {
                    decryption_error("Owl");
                    continue;
                }
            }
        }

        let mut handle = match File::open(key_file.clone()) {
            Ok(v) => v,
            Err(_) => {
                decryption_error("Dolphin");
                continue;
            }
        };

        loop {
            match handle.read(&mut data) {
                Ok(0) => break,
                Ok(n) => {
                    for val in data.iter().take(n) {
                        key_data.push(*val)
                    }

                    if n < READ_SIZE {
                        break;
                    }
                }
                Err(_) => {
                    decryption_error("Dog");
                    continue;
                }
            }
        }

        if final_data.len() > key_data.len() {
            decryption_error("Tiger");
            continue;
        }

        for i in 0..final_data.len() {
            final_data[i] ^= key_data[i];
        }

        let mut decrypted_file = match File::create(format!(
            "{}/{}",
            data_file_path.to_str().unwrap(),
            data_file_stem.to_str().unwrap(),
        )) {
            Ok(v) => v,
            Err(_) => {
                decryption_error("Rat");
                continue;
            }
        };

        match std::fs::remove_file(&data_file_buff) {
            Ok(_) => {}
            Err(_) => {
                decryption_error("Goose");
                continue;
            }
        }

        match std::fs::remove_file(key_file_buff) {
            Ok(_) => {}
            Err(_) => {
                decryption_error("Mouse");
                continue;
            }
        }

        decrypted_file.write_all(&final_data).unwrap();
    }
}

fn help_msg() {
    println!("=== Trilobyte v{} ===", TRILOBYTE_VERSION);
    println!(" -h | --help                                       : Prints this message.");
    println!(" -g | --generate (size) (filename)                 : Generate an encryption key of a given `size` and save out to a file.");
    println!(" -d | --decrypt (File to decrypt) (Decryption Key) : Decrypts a given encrypted file using a provided decryption key.");
    println!(" -e | --encrypt file1 [file2 [file3 ...]]          : Encrypts one or more provided files, and outputs the decryption keys.");
    println!(" -E | --Encrypt (file) (key)                       : Encrypts the provided files with the given encryption key.");

    exit(0);
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    for a in &args {
        match a.as_str() {
            "-h" | "--help" => help_msg(),
            _ => {}
        }
    }

    let mut arg_count = 0;
    let mut encrypting = false;

    let mut files_to_encrypt = vec![];

    let mut files_to_decrypt = vec![];

    let mut files_to_encrypt_static = vec![];

    let mut keys_to_generate = vec![];

    for i in 1..args.len() {
        if encrypting {
            if args[i].starts_with("-") {
                encrypting = false;
            } else {
                files_to_encrypt.push(args[i].clone());
                continue;
            }
        }

        if arg_count > 0 {
            arg_count -= 1;
            continue;
        }

        match args[i].as_str() {
            "-g" | "--generate" => {
                arg_count += 2;

                if i + 1 >= args.len() {
                    println!("Did not provide key size.");
                    exit(1);
                } else if i + 2 >= args.len() {
                    println!("Did not provide output destination.");
                    exit(1);
                }

                let mut first = true;

                match args[i + 1].parse::<usize>() {
                    Ok(_) => {}
                    Err(_) => first = false,
                }

                let file_index;
                let size_index;
                if first {
                    file_index = 2;
                    size_index = 1;
                } else {
                    file_index = 1;
                    size_index = 2;
                }

                let size = match args[i + size_index].parse::<usize>() {
                    Ok(v) => v,
                    Err(_) => {
                        generation_error("Sheep");
                        continue;
                    }
                };
                keys_to_generate.push((size, args[i + file_index].clone()));
            }

            "-d" | "--decrypt" => {
                arg_count += 2;

                if i + 1 >= args.len() {
                    println!("Did not provide file to decrypt.");
                    exit(1);
                } else if i + 2 >= args.len() {
                    println!("Did not provide decryption key for: {}", args[i + 1]);
                    exit(1);
                }

                if args[i + 1].ends_with(".cef") && args[i + 2].ends_with(".csd") {
                    files_to_decrypt.push((args[i + 2].clone(), args[i + 1].clone()));
                } else if args[i + 1].ends_with(".csd") && args[i + 2].ends_with(".cef") {
                    files_to_decrypt.push((args[i + 1].clone(), args[i + 2].clone()));
                } else {
                    eprintln!("Did not provide a `.cef` and `.csd` file pair.");
                }
            }

            "-E" | "--Encrypt" => {
                arg_count += 2;

                if i + 1 >= args.len() {
                    println!("Did not provide file to encrypt.");
                    exit(1);
                } else if i + 2 >= args.len() {
                    println!("Did not provide encryption key for: {}", args[i + 1]);
                    exit(1);
                }

                if args[i + 1].ends_with(".cef") {
                    files_to_encrypt_static.push((args[i + 2].clone(), args[i + 1].clone()));
                } else if args[i + 2].ends_with(".cef") {
                    files_to_encrypt_static.push((args[i + 1].clone(), args[i + 2].clone()));
                } else {
                    eprintln!("Did not provide a `.cef` and `.csd` file pair.");
                }
            }

            "-e" | "--encrypt" => {
                encrypting = true;
            }

            e => {
                eprintln!("Unknown argument: `{}`\nExiting..", e);
                exit(1);
            }
        }
    }

    if !files_to_encrypt.is_empty() {
        otp_encrypt(files_to_encrypt);
    }

    if !files_to_decrypt.is_empty() {
        otp_decrypt(files_to_decrypt);
    }

    if !keys_to_generate.is_empty() {
        otp_key_create(keys_to_generate);
    }

    if !files_to_encrypt_static.is_empty() {
        otp_encrypt_static(files_to_encrypt_static);
    }
}
