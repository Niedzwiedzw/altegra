pub use calamine::{
    open_workbook, DataType, Error as CalamineError, Range, RangeDeserializerBuilder, Reader, Xls,
};
use std::path::Path;

fn document_range(path: &Path) -> Range<DataType> {
    let mut excel: Xls<_> = open_workbook(path).unwrap();
    let sheet_name = excel.sheet_names().first().unwrap().clone();
    let range = excel.worksheet_range(&sheet_name).unwrap().unwrap();

    range
}

fn get_rows_raw(path: &Path) -> Result<Vec<crate::models::IntegraExcelRow>, CalamineError> {
    let range = document_range(path);
    let iter = RangeDeserializerBuilder::new().from_range(&range)?;
    let rows: Vec<crate::models::IntegraExcelRow> = iter
        .map(|e| {
            e.expect(
                "Zmieniła się struktura dokumentu programu Integra. \
                 Skontaktuj się z pomocą techniczną programu Alternator.",
            )
        })
        .collect();
    Ok(rows)
}
pub type EntriesIterator = Box<dyn Iterator<Item=crate::models::IntegraEntry>>;
pub fn get_rows(path: &Path) -> Result<
    EntriesIterator,
    CalamineError,
> {
    Ok(box get_rows_raw(path)?
        .into_iter()
        .map(|e| e.into()))
}
