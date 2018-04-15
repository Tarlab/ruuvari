//! Events sent by Beacon Scanner Android software
//! 
//! https://github.com/Bridouille/android-beacon-scanner

use event::{self, Event, ToRuuvariEvent};

use serde_json::Value;

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
    last_seen: usize,
    manufacturer: usize,
    rssi: isize,
    ruuvi_data: RuuviData,
    tx_power: isize,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuuviData {
    air_pressure: usize,
    humidity: usize,
    temperature: isize,
}

impl ToRuuvariEvent for Beacons {
    fn from(&self) -> event::Result<Vec<Event>> {
        fn to_event(beacon: &Beacon) -> Event {
            Event {
                beacon_address: beacon.beacon_address.clone(),
                air_pressure: beacon.ruuvi_data.air_pressure,
                humidity: beacon.ruuvi_data.humidity,
                temperature: beacon.ruuvi_data.temperature,
                rssi: beacon.rssi,
            }
        };

        Ok(self.beacons.iter().map(to_event).collect())
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
}
