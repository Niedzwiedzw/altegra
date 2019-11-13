#![feature(box_syntax)]

mod locator;
mod models;
mod parser;

use crate::locator::integra_files;
fn main() -> Result<(), crate::parser::CalamineError> {
    println!("\n###\nZlecenia\n");
//    for file in integra_files::<models::OrderEntry>() {
//        for row in file.into_iter().take(1) {
//            println!("{}", row.to_json().unwrap());
//        }
//    }

    println!("\n###\nSamochody\n");
    for file in integra_files::<models::VehicleEntry>() {
        for row in file.into_iter().take(1) {
            println!("{}", row.to_json().unwrap());
        }
    }

    Ok(())
}
