pub use calamine::{
    open_workbook, DataType, Error as CalamineError, Range, RangeDeserializerBuilder, Reader, Xls,
};
use std::path::Path;
use crate::models::{XLSEntry, OrderIntegraExcelRow, OrderEntry};
use calamine::RangeDeserializer;

pub fn document_range(path: &Path) -> Option<Range<DataType>> {
    let mut excel: Xls<_> = open_workbook(path).ok()?;
    let sheet_name = excel.sheet_names().first()?.clone();
    let range = excel.worksheet_range(&sheet_name)?.ok()?;

    Some(range)
}

pub fn get_rows_raw(path: &Path) -> Option<Vec<OrderIntegraExcelRow>> {
    let range = match document_range(path) {
        Some(r) => r,
        None => {
            println!("[error] failed getting the document range");
            return None;
        },
    };
    let mut iter = RangeDeserializerBuilder::new().from_range(&range).ok()?;
    let mut rows: Vec<OrderIntegraExcelRow> = vec![];
    for row in iter {
        match row {
            Ok(r) => rows.push(r),
            _ => {},
        }
    }
    if rows.is_empty() { return None; }
    Some(rows)
}

//pub type EntriesIterator = Box<dyn Iterator<Item=crate::models::OrderEntry>>;
pub fn get_rows(path: &Path) -> Option<Vec<OrderEntry>> {
    match get_rows_raw(path) {
        Some(r) => {
            let rows: Vec<OrderEntry> = r.into_iter()
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
