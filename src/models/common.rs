use serde::de::DeserializeOwned;

pub trait XLSEntry: Sized {
    type Raw: Into<Self> + DeserializeOwned;
    fn validate(&self) -> bool;
}
