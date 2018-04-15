use std::result;

#[derive(Debug)]
pub struct Event {
    pub beacon_address: String,
    pub air_pressure: usize,
    pub humidity: usize,
    pub temperature: isize,
    pub rssi: isize,
}

#[derive(Debug)]
pub enum Error {
    Other,
}

pub type Result<T> = result::Result<T, Error>;

pub trait ToRuuvariEvent {
    /// Convert a thing into vector of events.
    fn from(&self) -> Result<Vec<Event>>;
}
