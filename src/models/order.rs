use serde::{Deserialize, Serialize};
use crate::models::XLSEntry;

pub type OrderIntegraExcelRow = (
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
);

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderEntry {
    status: String,
    typ: String,
    numer: String,
    data_zgloszenia: String,
    numer_rejestracyjny: String,
    marka: String,
    model: String,
    wersja: String,
    wartosc_brutto: String,
    serwisant: String,
    kontrahent: String,
    telefon: String,
    rk: String,
    rm: String,
    a: String,
}

impl OrderEntry {
    pub fn to_json(&self) -> Result<String, serde_json::error::Error> {
        serde_json::to_string(self)
    }
}

impl XLSEntry for OrderEntry {
    type Raw = OrderIntegraExcelRow;
}

impl From<<OrderEntry as XLSEntry>::Raw> for OrderEntry {
    fn from(entry: OrderIntegraExcelRow) -> Self {
        let (
            status,
            typ,
            numer,
            data_zgloszenia,
            numer_rejestracyjny,
            marka,
            model,
            wersja,
            wartosc_brutto,
            serwisant,
            kontrahent,
            telefon,
            rk,
            rm,
            a,
        ) = entry;

        return Self {
            status,
            typ,
            numer,
            data_zgloszenia,
            numer_rejestracyjny,
            marka,
            model,
            wersja,
            wartosc_brutto,
            serwisant,
            kontrahent,
            telefon,
            rk,
            rm,
            a,
        };
    }
}
