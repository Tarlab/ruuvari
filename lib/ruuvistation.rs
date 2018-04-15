//! Events sent by Ruuvi Station Android software
//!
//! https://github.com/ruuvi/com.ruuvi.station

use serde_json;

use event::{self, Event, ToRuuvariEvent};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tags {
    device_id: String,
    event_id: String,
    tag: Option<Tag>,
    tags: Option<Vec<Tag>>,
    time: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    accel_x: f32,
    accel_y: f32,
    accel_z: f32,
    default_background: u32,
    favorite: bool,
    /// in relative humidity
    humidity: f32,
    id: String,
    name: Option<String>,
    /// in hPA
    pressure: f32,
    raw_data_blob: Blob,
    rssi: isize,
    /// in °C
    temperature: f32,
    update_at: String,
    voltage: f32,
}

#[derive(Debug, Deserialize)]
pub struct Blob {
    blob: Vec<i32>,
}

impl ToRuuvariEvent for Tags {
    fn from_json(input: &str) -> event::Result<Vec<Event>> {
        let value: Self = serde_json::from_str(input)?;
        value.to_events()
    }

    fn to_events(&self) -> event::Result<Vec<Event>> {
        fn to_event(tag: &Tag) -> Event {
            Event {
                beacon_address: tag.id.clone(),
                air_pressure: tag.pressure.round() as usize,
                humidity: tag.humidity.round() as usize,
                temperature: tag.temperature.round() as isize,
                rssi: tag.rssi,
            }
        }

        if let Some(ref tag) = self.tag {
            return Ok(vec![to_event(tag)]);
        }

        if let Some(ref tags) = self.tags {
            return Ok(tags.iter().map(to_event).collect());
        }

        Err(event::Error::EmptyEvent)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_json_with_tag() {
        let raw = r##"{"deviceId":"854af65f-13db-4082-b07e-89129690d275","eventId":"9e6329dd-06eb-474c-9d1d-9b4373704a6d","tag":{"accelX":0.0,"accelY":0.0,"accelZ":0.0,"defaultBackground":1,"favorite":true,"gatewayUrl":"http://192.168.1.4:1337/","humidity":22.0,"id":"D7:58:D2:87:08:F8","name":"Devitagi","pressure":996.0,"rawDataBlob":{"blob":[4,44,20,0,-63,-64]},"rssi":-57,"temperature":20.0,"updateAt":"Apr 14, 2018 12:22:27 AM","url":"https://ruu.vi/#BCwUAMHAr","voltage":0.0},"time":"Apr 14, 2018 12:22:27 AM"}"##;
        let event: Tags = serde_json::from_str(raw).expect("serde_json::from_str");
        assert_eq!(event.time, "Apr 14, 2018 12:22:27 AM");
    }

    #[test]
    fn test_json_with_tags() {
        let raw = r##"{"deviceId":"854af65f-13db-4082-b07e-89129690d275","eventId":"8bdb7814-21fd-4bbe-b6aa-2be5f552c14a","tags":[{"accelX":0.0,"accelY":0.0,"accelZ":0.0,"defaultBackground":1,"favorite":true,"humidity":22.0,"id":"D7:58:D2:87:08:F8","name":"Devitagi","pressure":996.0,"rawDataBlob":{"blob":[4,44,19,0,-63,-64]},"rssi":-63,"temperature":19.0,"updateAt":"Apr 14, 2018 12:12:38 AM","url":"https://ruu.vi/#BCwTAMHAr","voltage":0.0}],"time":"Apr 14, 2018 12:12:38 AM"}"##;
        let event: Tags = serde_json::from_str(raw).expect("serde_json::from_str");
        assert_eq!(event.time, "Apr 14, 2018 12:12:38 AM");
    }

    #[test]
    fn test_json_with_multiple_tags() {
        let raw = r##"{"deviceId":"854af65f-13db-4082-b07e-89129690d275","eventId":"520a341b-49e7-49e6-b908-05a27da7d6ac","tags":[{"accelX":0.936,"accelY":0.26,"accelZ":-0.204,"defaultBackground":6,"favorite":true,"gatewayUrl":"","humidity":68.0,"id":"D1:D8:2A:09:D6:C1","name":"Humidori","pressure":1017.0,"rawDataBlob":{"blob":[4,-120,22,0,-55,-12]},"rssi":-74,"temperature":22.0,"updateAt":"Apr 15, 2018 21:16:17","url":"https://ruu.vi/#BIgWAMn0T","voltage":3.193},{"accelX":0.0,"accelY":0.0,"accelZ":0.0,"defaultBackground":1,"favorite":true,"gatewayUrl":"http://192.168.1.4:1337/","humidity":38.0,"id":"D7:58:D2:87:08:F8","name":"Devitagi","pressure":1017.0,"rawDataBlob":{"blob":[4,76,24,0,-55,-12]},"rssi":-56,"temperature":24.0,"updateAt":"Apr 15, 2018 21:16:17","url":"https://ruu.vi/#BEwYAMn0r","voltage":0.0}],"time":"Apr 15, 2018 21:16:17"}"##;
        let event: Tags = serde_json::from_str(raw).expect("serde_json::from_str");
        assert_eq!(event.tags.map(|l| l.len()), Some(2));
    }
}
