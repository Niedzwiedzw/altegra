#![feature(box_syntax)]

mod locator;
mod models;
mod parser;
mod normalization;
#[macro_use] extern crate lazy_static;

use crate::locator::integra_files;
use crate::models::alternator::AlternatorRequest;
use reqwest::{
    blocking::{Client, RequestBuilder, Request},
    Result as RequestResult,
    header,
    StatusCode,
};
use futures::TryStreamExt;
use std::error::Error;
use anyhow::Result;

const BACKEND_URL: &'static str = "http://127.0.0.1:8000/api/orders/import-integra-car/";

fn send_order(order_json: String) -> Result<()> {
    let client: Client = Client::new();
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    );

    let request: RequestBuilder = client.post(BACKEND_URL)
        .body(order_json)
        .headers(headers);

    let response = request.send()?;
    let status = response.status();
    if status.is_success() {
        return Ok(())
    } else {
        return Err(anyhow::Error::msg(format!("[{}] {}", status, response.text()?)));
    }
}

fn main() -> Result<(), crate::parser::CalamineError> {
    println!("\n###\nZlecenia\n");

    let vehicles = integra_files().flatten().collect::<Vec<_>>();
//    for file in integra_files::<models::OrderEntry>() {
//        for row in file.into_iter().take(5) {
//            println!("{:?}", AlternatorRequest::from(&row, &vehicles));
//        }
//    }
    let valid_orders: Vec<AlternatorRequest> = integra_files::<models::OrderEntry>()
        .flatten()
        .filter_map(|entry| AlternatorRequest::from(&entry, &vehicles))
        .collect();

    for order in valid_orders.iter() {
        let json = serde_json::to_string(order).unwrap();
        match send_order(json) {
            Ok(_) => println!("[OK] {}", order.vehicle.plates_number),
            Err(e) => println!("[ERROR!] \nentry: {:#?}\nmessage: {}", order, e),
        }
    }

    Ok(())
}
