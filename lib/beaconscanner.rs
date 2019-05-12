//! Events sent by Beacon Scanner Android software
//!
//! https://github.com/Bridouille/android-beacon-scanner

use chrono::{Utc, Local, TimeZone};
use serde_json::{self, Value};

use event::{self, Event, ToRuuvariEvent};

#[derive(Debug, Deserialize)]
pub struct Beacons {
    beacons: Vec<Beacon>,
    reader: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Beacon {
    beacon_address: String,
    beacon_type: String,
    distance: f64,
    eddystone_url_data: Value,
    hashcode: usize,
    is_blocked: bool,
    last_minute_seen: usize,
    /// Milliseconds since 1970-01-01 00:00:00 local time
    last_seen: usize,
    manufacturer: usize,
    rssi: isize,
    ruuvi_data: RuuviData,
    tx_power: isize,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuuviData {
    /// in hPA
    air_pressure: f32,
    /// in relative humidity
    humidity: f32,
    /// in °C
    temperature: f32,
}

impl ToRuuvariEvent for Beacons {
    fn from_json(input: &str) -> event::Result<Vec<Event>> {
        let value: Self = serde_json::from_str(input)?;
        value.to_events()
    }

    fn to_events(&self) -> event::Result<Vec<Event>> {
        let events: Vec<Event> = self.beacons.iter().map(to_event).collect();

        if events.is_empty() {
            return Err(event::Error::EmptyEvent);
        }

        Ok(events)
    }
}

fn to_event(beacon: &Beacon) -> Event {
    let seconds = (beacon.last_seen / 1000) as i64;
    let localtime = Local.timestamp(seconds, 0);

    Event {
        beacon_address: beacon.beacon_address.clone(),
        air_pressure: beacon.ruuvi_data.air_pressure,
        humidity: beacon.ruuvi_data.humidity,
        temperature: beacon.ruuvi_data.temperature,
        rssi: beacon.rssi,
        timestamp: localtime.with_timezone(&Utc),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_json() {
        let raw = r##"{"beacons":[{"beaconAddress":"D7:58:D2:87:08:F8","beaconType":"ruuvitag","distance":2.5337382706296463,"eddystoneUrlData":{"url":"https://ruu.vi/#BCwVAMCUr"},"hashcode":1141403717,"isBlocked":false,"lastMinuteSeen":25396428,"lastSeen":1523785721504,"manufacturer":65194,"rssi":-60,"ruuviData":{"airPressure":993,"humidity":22,"temperature":21},"txPower":-48}],"reader":"Scanner 1"}"##;
        let event: Beacons = serde_json::from_str(raw).expect("serde_json::from_str");
        assert_eq!(event.reader, "Scanner 1");
    }

    #[test]
    fn test_json_multiple_beacons() {
        let raw = r##"{"beacons":[{"beaconAddress":"D7:58:D2:87:08:F8","beaconType":"ruuvitag","distance":1.939022861124338,"eddystoneUrlData":{"url":"https://ruu.vi/#BDQXAMn0r"},"hashcode":1141403717,"isBlocked":false,"lastMinuteSeen":25396939,"lastSeen":1523816361122,"manufacturer":65194,"rssi":-57,"ruuviData":{"airPressure":1017,"humidity":26,"temperature":23},"txPower":-48},{"beaconAddress":"D1:D8:2A:09:D6:C1","beaconType":"ruuvitag","distance":15.18942027557396,"eddystoneUrlData":{"url":"https://ruu.vi/#BIgWAMn0T"},"hashcode":984684823,"isBlocked":false,"lastMinuteSeen":25396939,"lastSeen":1523816361120,"manufacturer":65194,"rssi":-81,"ruuviData":{"airPressure":1017,"humidity":68,"temperature":22},"txPower":-48}],"reader":"Scanner 1"}"##;
        let event: Beacons = serde_json::from_str(raw).expect("serde_json::from_str");
        assert_eq!(event.beacons.len(), 2);
    }

    #[test]
    fn test_json_to_event1() {
        let raw = r##"{"beacons":[{"beaconAddress":"D7:58:D2:87:08:F8","beaconType":"ruuvitag","distance":2.5337382706296463,"eddystoneUrlData":{"url":"https://ruu.vi/#BCwVAMCUr"},"hashcode":1141403717,"isBlocked":false,"lastMinuteSeen":25396428,"lastSeen":1523785721504,"manufacturer":65194,"rssi":-60,"ruuviData":{"airPressure":993,"humidity":22,"temperature":21},"txPower":-48}],"reader":"Scanner 1"}"##;
        let event: Vec<Event> = Beacons::from_json(&raw).expect("from_json");
        assert_eq!(event.len(), 1);

    }
}
