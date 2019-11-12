#![feature(box_syntax)]

mod locator;
mod models;
mod parser;

use crate::locator::integra_files;
fn main() -> Result<(), crate::parser::CalamineError> {
    for file in integra_files::<models::OrderEntry>() {
        for row in file.into_iter().take(5) {
            println!("{}", row.to_json().unwrap());
        }
    }

    for file in integra_files::<models::VehicleEntry>() {
        for row in file.into_iter().skip(1).take(5) {
            println!("{}", row.to_json().unwrap());
        }
    }

    Ok(())
}
