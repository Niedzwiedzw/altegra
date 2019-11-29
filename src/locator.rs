use dirs::desktop_dir;
use walkdir::{WalkDir, DirEntry};

fn locate_dump_files() -> Box<dyn Iterator<Item=DirEntry>> {
    let desktop = desktop_dir().unwrap();
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
    box sorted.into_iter()
}

//pub fn integra_files() -> std::iter::FilterMap<std::boxed::Box<dyn std::iter::Iterator<Item = walkdir::dent::DirEntry>> {
pub fn integra_files<T>() -> Box<dyn Iterator<Item=Vec<T>>>
    where T: crate::models::common::XLSEntry {
    let xls_files: Vec<_> = locate_dump_files().collect();

    let mut _integra_files = xls_files
        .into_iter()
        .filter_map(|entry| crate::parser::get_rows::<T>(entry.path()));

    box _integra_files
}
