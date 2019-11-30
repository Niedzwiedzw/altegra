use std::{
    path::{PathBuf},
    fs::read_dir,
};
use std::fs::{ReadDir, DirEntry};
use std::error::Error;
use crate::parser::{document_range, get_rows_raw, get_rows};
use crate::models;

fn test_data_dir() -> PathBuf {
    PathBuf::from("./test_data")
}

fn test_files() -> Box<dyn Iterator<Item=DirEntry>> {
    box read_dir(test_data_dir())
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name().to_str().unwrap().ends_with(".xls"))
        .into_iter()
}

fn test_file(file: DirEntry) -> Result<(), Box<dyn Error>> {
    println!("[{:?}]", file);
    let path = file.path().clone();
    print!("document range: ");
    assert!(document_range(path.as_path()).is_some());
    print!("ok\n");

    print!("get_rows_raw: ");
    assert!(get_rows_raw(path.as_path()).is_some());
    print!("ok\n");

    print!("get_rows: ");
    assert!(get_rows(path.as_path()).is_some());


    print!("ok\n");
    Ok(())
}

pub fn test_data() -> Result<(), Box<dyn Error>> {
    println!("testing!");
    println!("searching in directory: {:?}", test_data_dir());
    for file in test_files() {
        test_file(file)?;
    }
    Ok(())
}
