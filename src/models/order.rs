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
    String,
);

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderEntry {
    pub status: String,
    pub typ: String,
    pub numer: String,
    pub data_zgloszenia: String,
    pub numer_rejestracyjny: String,
    pub marka: String,
    pub model: String,
    pub wersja: String,
    pub wartosc_brutto: String,
    pub serwisant: String,
    pub kontrahent: String,
    pub telefon: String,
    pub rk: String,
    pub rm: String,
    pub a: String,
    pub serwis: String,
}

impl XLSEntry for OrderEntry {
    type Raw = OrderIntegraExcelRow;
    fn validate(&self) -> bool {
        true
    }
}

impl From<<OrderEntry as XLSEntry>::Raw> for OrderEntry { // TODO: order entry is wrong
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
            kontrahent,
            serwisant,
            telefon,
            wartosc_brutto,
            rk,
            rm,
            a,
            serwis,
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
            serwis,
        };
    }
}
