use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use crate::models::XLSEntry;

pub type OrderIntegraExcelRow = [String; 18];

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
    type Raw = OrderIntegraExcelRow;
    fn validate(&self) -> bool {
        true
    }
}

impl From<OrderIntegraExcelRow> for OrderEntry {
    // TODO: order entry is wrong
    fn from(entry: OrderIntegraExcelRow) -> Self {
//        let (
//            numer,
//            numer_nadwozia,
//            data_zgloszenia,
//            numer_rejestracyjny,
//            marka,
//            rok_produkcji,
//            kolor,
//            model,
//            wersja,
//            nip,
//            kontrahent,
//            serwisant,
//            telefon,
//            wartosc_brutto,
//            stan_licznika,
//            rk,
//            rm,
//            a,
//        ) = entry;
        let numer = entry[0].clone();
        let numer_nadwozia = entry[1].clone();
        let data_zgloszenia = entry[2].clone();
        let numer_rejestracyjny = entry[3].clone();
        let marka = entry[4].clone();
        let rok_produkcji = entry[5].clone();
        let kolor = entry[6].clone();
        let model = entry[7].clone();
        let wersja = entry[8].clone();
        let nip = entry[9].clone();
        let kontrahent = entry[10].clone();
        let serwisant = entry[11].clone();
        let telefon = entry[12].clone();
        let wartosc_brutto = entry[13].clone();
        let stan_licznika = entry[14].clone();
        let rk = entry[15].clone();
        let rm = entry[16].clone();
        let a = entry[17].clone();

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
