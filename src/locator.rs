use dirs::desktop_dir;
use walkdir::{WalkDir, DirEntry};
use crate::models::{OrderIntegraExcelRow, OrderEntry};
use crate::is_debug;

fn locate_dump_files() -> Box<dyn Iterator<Item=DirEntry>> {
    let desktop = desktop_dir().expect("Nie udało się zlokalizować ścieżki do pulpitu.");
    if is_debug() {
        println!("szukam w: {:?}", desktop);
    }
    let walkdir = WalkDir::new(desktop);
    let filtered = walkdir
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name().to_str().unwrap().ends_with(".xls"));
    let mut sorted = filtered.collect::<Vec<_>>();
    sorted.sort_by(|one, other| {
            one.metadata()
                .unwrap()
                .created()
                .unwrap()
                .cmp(&other.metadata()
                    .unwrap()
                    .created()
                    .unwrap()
                )
        }
    );
    if is_debug() {
        println!("znaleziono pliki: {:?}", sorted);
    }
    box sorted.into_iter()
}

//pub fn integra_files() -> std::iter::FilterMap<std::boxed::Box<dyn std::iter::Iterator<Item = walkdir::dent::DirEntry>> {
pub fn integra_files() -> Box<dyn Iterator<Item=Vec<OrderEntry>>> {
    let xls_files: Vec<_> = locate_dump_files().collect();

    let mut _integra_files = xls_files
        .into_iter()
        .filter_map(|entry| crate::parser::get_rows(entry.path()));

    box _integra_files
}
