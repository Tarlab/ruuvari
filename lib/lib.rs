#[macro_use]
extern crate serde_derive;
extern crate serde_json;

/// A common event for everything
pub mod event;
pub use event::{Event, ToRuuvariEvent};

/// Support for Ruuvi Station
pub mod ruuvistation;

/// Support for Beacon Scanner
pub mod beaconscanner;
