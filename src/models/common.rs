use serde::de::DeserializeOwned;

pub trait XLSEntry: Sized {
    fn validate(&self) -> bool;
}
