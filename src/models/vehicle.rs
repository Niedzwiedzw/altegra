use serde::{Deserialize, Serialize};
use crate::models::XLSEntry;
use regex::Regex;

pub type VehicleIntegraExcelRow = (
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
pub struct VehicleEntry {
    pub numer_rejestracyjny: String,
    pub marka: String,
    pub model: String,
    pub wersja: String,
    pub silnik: String,
    pub rok_produkcji: String,
    pub nr_nadwozia: String,
    pub kolor: String,
    pub uzytkownik_pojazdu: String,
    pub miejscowosc: String,
    pub telefon: String,
    pub miejscowosc_wlasciciela: String,
    pub komunikat: String,
    pub uwagi: String,
}

impl XLSEntry for VehicleEntry {
    type Raw = VehicleIntegraExcelRow;
    fn validate(&self) -> bool {
        lazy_static! {
            static ref CONTAINS_NUMBER: Regex = Regex::new(r".*?\d.*?").unwrap();
        }
        CONTAINS_NUMBER.is_match(&self.rok_produkcji)
    }
}

impl From<VehicleIntegraExcelRow> for VehicleEntry {
    fn from(entry: VehicleIntegraExcelRow) -> Self {
        let (
            nr_rejestracyjny,
            marka,
            model,
            wersja,
            silnik,
            rok_produkcji,
            nr_nadwozia,
            kolor,
            uzytkownik_pojazdu,
            miejscowosc,
            telefon,
            miejscowosc_wlasciciela,
            komunikat,
            uwagi,
        ) = entry;

        return VehicleEntry {
            numer_rejestracyjny: nr_rejestracyjny,
            marka,
            model,
            wersja,
            silnik,
            rok_produkcji,
            nr_nadwozia,
            kolor,
            uzytkownik_pojazdu,
            miejscowosc,
            telefon,
            miejscowosc_wlasciciela,
            komunikat,
            uwagi,
        }
    }
}
