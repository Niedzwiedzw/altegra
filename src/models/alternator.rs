use serde::{Deserialize, Serialize};
use crate::models::{OrderEntry};
use crate::normalization::{normalize_phone_number, normalize_date};

// expected example data data:
//dummy_data = {
//    'user': {
//        'first_name': 'Wojciech',
//        'last_name': 'Brożek',
//        'phone_number': '886894737',  # with the phone number
//},
//    'vehicle': {
//        'engine': '1.9 TDI',
//        'color': 'niebieski',
//        'engine_code': 'WX12314122ASX',
//        'plates_number': 'CAL 19930213',
//        'brand': 'Alfa Romeo',
//        'model': 'Classic',
//        'manufacture_date': '1993-02-13T00:00',
//    },
//}
#[derive(Debug, Serialize, Deserialize)]
pub struct AlternatorUser {
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,
}

impl AlternatorUser {
    pub fn validate(&self) -> Result<(), ()> {
        Ok(())
    }
    pub fn normalize(&mut self) {
        self.phone_number = normalize_phone_number(self.phone_number.clone());
    }

    pub fn build(
        full_name: String,
        phone_number: String,
    ) -> Option<Self> {
        let mut full_name = full_name.split(" ");
        let first_name = String::from(full_name.next()?.clone());
        let last_name = full_name.collect::<Vec<_>>().join(" ");

        let mut user = Self { first_name, last_name, phone_number };
        user.normalize();
        match user.validate() {
            Ok(_) => Some(user),
            _ => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlternatorVehicle {
    pub engine: String,
    pub color: String,
    pub engine_code: String,
    pub plates_number: String,
    pub brand: String,
    pub model: String,
    pub manufacture_date: String,
}

impl AlternatorVehicle {
    pub fn validate(&self) -> Result<(), ()> {
        Ok(())
    }
    pub fn normalize(&mut self) -> Result<(), ()> {
        self.manufacture_date = normalize_date(self.manufacture_date.clone())?;
        Ok(())
    }
    pub fn build(
        engine: String,
        color: String,
        engine_code: String,
        plates_number: String,
        brand: String,
        model: String,
        manufacture_date: String,
    ) -> Option<Self> {
        let mut vehicle = Self {
            engine_code,
            engine,
            color,
            plates_number,
            brand,
            model,
            manufacture_date,
        };

        match vehicle.normalize() {
            Ok(_) => {},
            _ => return None,
        };
        match vehicle.validate() {
            Ok(_) => Some(vehicle),
            _ => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlternatorRequest {
    pub user: AlternatorUser,
    pub vehicle: AlternatorVehicle,
}

impl AlternatorRequest {
    pub fn from(order: &OrderEntry) -> Option<Self> {
        let full_name = order.kontrahent.clone();
        let phone_number = order.telefon.clone();

        let user = AlternatorUser::build(full_name, phone_number)?;


        let engine = order.wersja.clone();
        let plates_number = order.numer_rejestracyjny.clone();
        let brand = order.marka.clone();
        let model = order.model.clone();
        let engine_code = order.numer_nadwozia.clone();
        let manufacture_date = order.rok_produkcji.clone();
        let color = order.kolor.clone();

        let vehicle = AlternatorVehicle::build(
            engine,
            color,
            engine_code,
            plates_number,
            brand,
            model,
            manufacture_date,
        )?;

        Some(Self { user, vehicle })
    }
}