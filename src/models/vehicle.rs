use serde::{Deserialize, Serialize};
use crate::models::XLSEntry;

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
    nr_rejestracyjny: String,
    marka: String,
    model: String,
    wersja: String,
    silnik: String,
    rok_produkcji: String,
    nr_nadwozia: String,
    kolor: String,
    uzytkownik_pojazdu: String,
    miejscowosc: String,
    telefon: String,
    miejscowosc_wlasciciela: String,
    komunikat: String,
    uwagi: String,
}

impl VehicleEntry {
    pub fn to_json(&self) -> Result<String, serde_json::error::Error> {
        serde_json::to_string(self)
    }
}

impl XLSEntry for VehicleEntry {
    type Raw = VehicleIntegraExcelRow;
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
        }
    }
}
