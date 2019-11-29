use dirs::desktop_dir;
use std::path::{PathBuf};
use std::fs::{create_dir_all, read_dir, File, OpenOptions};
use std::io::{Read, Write};
use std::collections::HashSet;
use crate::normalization::remove_whitespace;

pub static ALTEGRA_SUBDIR: &'static str = "altegra";
pub static ALTEGRA_CACHE_FILE: &'static str = "altegra.cache";
pub static ALTEGRA_CREDENTIALS_FILE: &'static str = "login.txt";


pub fn altegra_path() -> PathBuf {
    PathBuf::from(
        desktop_dir()
            .expect("Nie udało się stworzyć struktury katalogów")
    ).join(PathBuf::from(ALTEGRA_SUBDIR))
}

pub fn altegra_file_path(filename: &'static str) -> PathBuf {
    altegra_path().join(PathBuf::from(filename))
}

pub fn cache_file() -> PathBuf {
    altegra_file_path(ALTEGRA_CACHE_FILE)
}

pub fn credentials_file() -> PathBuf {
    altegra_file_path(ALTEGRA_CREDENTIALS_FILE)
}

pub fn get_token() -> String {
    let mut token = String::new();
    File::open(credentials_file())
        .expect("Nie udało się otworzyć pliku z hasłem dostępu.")
        .read_to_string(&mut token)
        .expect("Nie udało się otworzyć pliku z hasłem dostępu.");
    let token= remove_whitespace(token);
    token
}

pub fn read_cache() -> Vec<String> {
    let mut cache = String::new();
    File::open(cache_file())
        .expect("Nie udało się otworzyć pliku, skontaktuj się z administratorem.")
        .read_to_string(&mut cache)
        .expect("Nie udało się odczytać pliku, skontaktuj się z administratorem.");

    cache.lines()
        .filter(|e| !e.is_empty())
        .map(|line| String::from(line))
        .collect()
}

pub fn write_cache(checksum: String) {
    let mut current_entries: HashSet<String> = read_cache().into_iter().collect();
    current_entries.insert(checksum);

    let mut file = OpenOptions::new()
        .write(true)
        .open(cache_file())
        .expect("Nie udało się otworzyć pliku, skontaktuj się z administratorem.");
    for entry in current_entries {
        writeln!(file, "{}", entry)
            .expect("Nie udało się napisać do pliku, skontaktuj się z administratorem.");
    }
}

pub fn recreate_altegra_path() -> std::io::Result<()> {
    create_dir_all(altegra_path())?;
    let mut files = read_dir(altegra_path())?;

    let mut create_file = |filename: &'static str| {
        let filename = filename.clone();
        match files.find(|e| e.as_ref().unwrap().file_name() == filename) {
            None => {
                println!("{} file not found, creating one", filename);
                File::create(altegra_file_path(filename))
                    .expect("failed to create file");
            },
            _ => {},
        }
    };

    for filename in &[ALTEGRA_CACHE_FILE, ALTEGRA_CREDENTIALS_FILE] {
        create_file(filename);
    }

    Ok(())
}
