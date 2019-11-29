use chrono::prelude::*;

pub fn remove_whitespace(input: String) -> String {
    input.split_whitespace().collect::<Vec<_>>().join("")
}

pub fn normalize_phone_number(phone_number: String) -> String {
    let mut phone_number = remove_whitespace(phone_number);
    if !phone_number.starts_with("+48") {
        phone_number = format!("{}{}", "+48", phone_number);
    }
    phone_number
}

pub fn normalize_date(date: String) -> Result<String, ()> {
    let date = remove_whitespace(date);
    match date.len() {
        4 => Ok(
            NaiveDate::from_yo(
                date
                    .parse()
                    .map_err(|_e| ())?,
                1,
            )
                .format("%Y-%m-%dT00:00")
                .to_string(),
        ),
        _ => Err(()),
    }
}
