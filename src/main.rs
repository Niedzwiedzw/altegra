#![feature(box_syntax)]

mod locator;
mod models;
mod parser;

fn main() -> Result<(), crate::parser::CalamineError> {
    for file in crate::locator::integra_files() {
        for row in file.into_iter().take(1) {
            println!("{}", row.to_json().unwrap());
        }
    }

    Ok(())
}
