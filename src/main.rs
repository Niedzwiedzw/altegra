#![feature(box_syntax)]

#[macro_use]
extern crate lazy_static;

mod locator;
mod models;
mod parser;
mod normalization;
mod cache;

use crate::locator::integra_files;
use crate::models::alternator::AlternatorRequest;
use reqwest::{
    blocking::{Client, RequestBuilder},
    Result as RequestResult,
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

const DEBUG_BACKEND_URL: &'static str = "http://aplikacja-alternator.pl/api/orders/import-integra-car/";
const BACKEND_URL: &'static str = "http://127.0.0.1:8000/api/orders/import-integra-car/";

pub fn is_debug() -> bool {
    args().last().unwrap() == "--debug"
}

pub fn backend_url() -> &'static str {
    match is_debug() {
        true => DEBUG_BACKEND_URL,
        false => BACKEND_URL,
    }
}

fn send_order(order_json: String) -> Result<()> {
    let client: Client = Client::new();
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    );

    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(get_token().as_str())
            .expect("Nie udało się odczytać hasła dostępu."),
    );

    let request: RequestBuilder = client
        .post(backend_url())
        .body(order_json)
        .headers(headers);

    let response = request.send()?;
    let status = response.status();
    if status.is_success() {
        return Ok(());
    } else {
        return Err(anyhow::Error::msg(format!("[{}] {}", status, response.text()?)));
    }
}

fn main() -> Result<(), crate::parser::CalamineError> {
    println!(">><><< Altegra >><><<");
    recreate_altegra_path().expect("nie udało sie stworzyć struktury katalogów");
    loop {
        let vehicles = integra_files().flatten().collect::<Vec<_>>();
        let valid_orders: Vec<AlternatorRequest> = integra_files::<models::OrderEntry>()
            .take(1)
            .flatten()
            .filter_map(|entry| AlternatorRequest::from(&entry, &vehicles))
            .collect();

        let cache: HashSet<_> = read_cache().into_iter().collect();

        for order in valid_orders.iter() {
            let json = serde_json::to_string(order).unwrap();
            let checksum = format!("{:x}", md5::compute(json.clone()));
            if cache.contains(&checksum) {
                continue;
            }
            match send_order(json) {
                Ok(_) => println!("[OK] {}", order.vehicle.plates_number),
                Err(e) => println!(
                    "[BŁĄD!] zlecenie: {:#?}: {}",
                    order.vehicle.plates_number,
                    e,
                ),
            }
            write_cache(checksum);
        }
        print!(".");
        std::io::stdout().flush().unwrap();
        sleep(Duration::from_secs(5));
    }

    Ok(())
}
