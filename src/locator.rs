use dirs::desktop_dir;
use walkdir::{WalkDir, DirEntry};
use crate::parser::{EntriesIterator};

fn locate_dump_files() -> Box<dyn Iterator<Item=DirEntry>> {
    let desktop = desktop_dir().unwrap();
    println!("{:?}", desktop);
    let walkdir = WalkDir::new(desktop);
    let filtered = walkdir
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name().to_str().unwrap().ends_with(".xls"));

    box filtered
}

//pub fn integra_files() -> std::iter::FilterMap<std::boxed::Box<dyn std::iter::Iterator<Item = walkdir::dent::DirEntry>> {
pub fn integra_files() -> Box<dyn Iterator<Item=EntriesIterator>> {
    let xls_files = locate_dump_files();
    let _integra_files = box xls_files
        .filter_map(|entry| crate::parser::get_rows(entry.path()).ok());
    _integra_files
}
