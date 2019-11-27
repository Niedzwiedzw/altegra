pub use calamine::{
    open_workbook, DataType, Error as CalamineError, Range, RangeDeserializerBuilder, Reader, Xls,
};
use std::path::Path;
use crate::models::XLSEntry;

fn document_range(path: &Path) -> Range<DataType> {
    let mut excel: Xls<_> = open_workbook(path).unwrap();
    let sheet_name = excel.sheet_names().first().unwrap().clone();
    let range = excel.worksheet_range(&sheet_name).unwrap().unwrap();

    range
}

fn get_rows_raw<T>(path: &Path) -> Option<Vec<<T as XLSEntry>::Raw>>
    where T: crate::models::common::XLSEntry {
    let range = document_range(path);
    let iter = RangeDeserializerBuilder::new().from_range(&range).unwrap();
    let mut rows = vec![];
    for row in iter {
        match row {
            Ok(r) => rows.push(r),
            _ => return None,
        }
    }
    Some(rows)
}

//pub type EntriesIterator = Box<dyn Iterator<Item=crate::models::OrderEntry>>;
pub fn get_rows<T>(path: &Path) -> Option<Vec<T>>
    where T: crate::models::common::XLSEntry {
    match get_rows_raw::<T>(path) {
        Some(r) => {
            let rows: Vec<T> = r.into_iter()
                .map(|e| e.into())
                .filter(|e| e.validate())
                .collect();
            if rows.is_empty() {
                return None;
            }
            return Some(rows);
        },
        _ => None,
    }
}
