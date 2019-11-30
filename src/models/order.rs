use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
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
    String,
    String,
);

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderEntry {
    pub numer: String,
    pub numer_nadwozia: String,
    pub data_zgloszenia: String,
    pub numer_rejestracyjny: String,
    pub marka: String,
    pub rok_produkcji: String,
    pub kolor: String,
    pub model: String,
    pub wersja: String,
    pub nip: String,
    pub kontrahent: String,
    pub serwisant: String,
    pub telefon: String,
    pub wartosc_brutto: String,
    pub stan_licznika: String,
    pub rk: String,
    pub rm: String,
    pub a: String,
}

impl XLSEntry for OrderEntry {
    fn validate(&self) -> bool {
        true
    }
}

impl From<OrderIntegraExcelRow> for OrderEntry {
    // TODO: order entry is wrong
    fn from(entry: OrderIntegraExcelRow) -> Self {
        let (
            numer,
            numer_nadwozia,
            data_zgloszenia,
            numer_rejestracyjny,
            marka,
            rok_produkcji,
            kolor,
            model,
            wersja,
            nip,
            kontrahent,
            serwisant,
            telefon,
            wartosc_brutto,
            stan_licznika,
            rk,
            rm,
            a,
        ) = entry;

        return Self {
            numer,
            numer_nadwozia,
            data_zgloszenia,
            numer_rejestracyjny,
            marka,
            rok_produkcji,
            kolor,
            model,
            wersja,
            nip,
            kontrahent,
            serwisant,
            telefon,
            wartosc_brutto,
            stan_licznika,
            rk,
            rm,
            a,
        };
    }
}
