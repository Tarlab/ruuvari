//! # A "one-event-to-rule-them-all" kind of way for RuuviTags
//!
//! Event contains information from single broadcast of single beacon. Different
//! software send different data and this `Event` tries it's best to contain the
//! common information.
//!
//! For climate monitoring the most important data is already present: air
//! pressure, humidity and temperature. Beacon address can be used to identify
//! the beacon in question.

use std::result;
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub beacon_address: String,
    /// in hPA
    pub air_pressure: usize,
    /// in relative humidity
    pub humidity: usize,
    /// in °C
    pub temperature: isize,
    pub rssi: isize,
}

#[derive(Debug)]
pub enum Error {
    /// Missing information to produce Event
    EmptyEvent,
    /// JSON error
    JSONError(serde_json::Error),
}

pub type Result<T> = result::Result<T, Error>;

/// A trait for converting received information into one or more Events
///
/// One HTTP POST JSON can contain information from one or more Beacons in same
/// message. This information is then dissected into one or more Events. One
/// Event per one broadcast from one beacon.
pub trait ToRuuvariEvent {
    /// Convert a JSON into vector of events.
    fn from_json(input: &str) -> Result<Vec<Event>>;

    /// Convert Self into vector of events
    fn to_events(&self) -> Result<Vec<Event>>;
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::JSONError(err)
    }
}
