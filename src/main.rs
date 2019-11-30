#![feature(box_syntax)]

#[macro_use]
extern crate lazy_static;

mod locator;
mod models;
mod parser;
mod normalization;
mod cache;
mod test_data;

use crate::locator::integra_files;
use crate::models::alternator::AlternatorRequest;
use reqwest::{
    blocking::{Client, RequestBuilder},
    header,
    StatusCode,
};
use anyhow::Result;
use std::env::args;
use crate::cache::{recreate_altegra_path, read_cache, write_cache, get_token};
use std::collections::HashSet;
use std::thread::sleep;
use std::time::Duration;
use std::io::Write;
use snafu::{ResultExt, Snafu};

const BACKEND_URL: &'static str = "http://aplikacja-alternator.pl/api/orders/import-integra-car/";
const DEBUG_BACKEND_URL: &'static str = "http://127.0.0.1:8000/api/orders/import-integra-car/";

pub fn is_debug() -> bool {
    args().last().expect("Brak ostatniego argumentu") == "--debug"
}

pub fn is_test() -> bool {
    args().last().expect("Brak ostatniego argumentu") == "--test"
}

pub fn backend_url() -> &'static str {
    match is_debug() {
        true => DEBUG_BACKEND_URL,
        false => BACKEND_URL,
    }
}

#[derive(Snafu, Debug)]
enum RequestError {
    #[snafu(display("Hasło logowania wygasło lub jest niepoprawne."))]
    WrongCredentials,
    #[snafu(display("Brak hasła logowania w login.txt"))]
    CredentialsNotPresent,
    #[snafu(display("Źle sformatowany wpis zlecenia: {}", message))]
    ValidationError { message: String },
}

type RequestResult<T, E = RequestError> = std::result::Result<T, E>;

fn send_order(order_json: String) -> RequestResult<()> {
    let client: Client = Client::new();
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    );
    let authorization = format!("Bearer {}", get_token());
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(authorization.as_str())
            .expect("Nie udało się odczytać hasła dostępu."),
    );

    if is_debug() {
        println!("[DEBUG] send_order() -> {}", backend_url());
    }

    let request: RequestBuilder = client
        .post(backend_url())
        .body(order_json)
        .headers(headers);

    let response = request.send().expect("Błąd sieciowy, brak połączenia z Internetem?");
    let status = response.status();
    if status == 401 {
        return Err(RequestError::WrongCredentials)
    }
    if status.is_success() {
        return Ok(());
    } else {
        return Err(
            RequestError::ValidationError {message: format!("[{}] {}", status, response.text().expect("Nie udało się odczytać odpowiedzi serwera"))},
        );
    }
}

fn main() -> Result<(), crate::parser::CalamineError> {
    println!(">><><< Altegra >><><<");
    if is_test() {
        test_data::test_data();
        return Ok(())
    }
    recreate_altegra_path().expect("nie udało sie stworzyć struktury katalogów");
    loop {
        let valid_orders: Vec<AlternatorRequest> = integra_files()
            .flatten()
            .filter_map(|entry| AlternatorRequest::from(&entry))
            .collect();

        let cache: HashSet<_> = read_cache().into_iter().collect();

        for order in valid_orders.iter() {
            let json = serde_json::to_string(order).expect("Nie udało się zserializować pliku integry");
            let checksum = format!("{:x}", md5::compute(json.clone()));
            if cache.contains(&checksum) {
                continue;
            }
            match send_order(json) {
                Ok(_) => println!("[OK] {}", order.vehicle.plates_number),
                Err(RequestError::WrongCredentials) => {
                    println!("Hasło w login.txt wygasło lub jest niepoprawne.");
                    break;
                },
                Err(e) => println!(
                    "[BŁĄD!] zlecenie: {:#?}: {}",
                    order.vehicle.plates_number,
                    e,
                ),
            }
            write_cache(checksum);
        }
        print!(".");
        match std::io::stdout().flush() {
            _ => {},
        };
        sleep(Duration::from_secs(5));
    }

    Ok(())
}
