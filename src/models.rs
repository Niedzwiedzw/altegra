use serde::{Deserialize, Serialize};

pub type IntegraExcelRow = (
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
pub struct IntegraEntry {
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

impl IntegraEntry {
    pub fn to_json(&self) -> Result<String, serde_json::error::Error> {
        serde_json::to_string(self)
    }
}

impl From<IntegraExcelRow> for IntegraEntry {
    fn from(entry: IntegraExcelRow) -> Self {
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
        return IntegraEntry {
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
