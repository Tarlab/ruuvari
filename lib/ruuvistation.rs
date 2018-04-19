//! Events sent by Ruuvi Station Android software
//!
//! https://github.com/ruuvi/com.ruuvi.station

use chrono::{DateTime, Utc, NaiveDateTime, TimeZone, Local};
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
        if let Some(ref tag) = self.tag {
            return Ok(vec![to_event(tag)?]);
        }

        if let Some(ref tags) = self.tags {
            let mut res = Vec::with_capacity(tags.len());
            for tag in tags {
                res.push(to_event(tag)?);
            }
            return Ok(res)
        }

        Err(event::Error::EmptyEvent)
    }
}

fn to_event(tag: &Tag) -> event::Result<Event> {
    Ok(Event {
        beacon_address: tag.id.clone(),
        air_pressure: tag.pressure,
        humidity: tag.humidity,
        temperature: tag.temperature,
        rssi: tag.rssi,
        timestamp: time_parser(&tag.update_at)?,
    })
}

fn time_parser(input: &str) -> event::Result<DateTime<Utc>> {
    // We have identified two different time stamp formats:
    //   Apr 14, 2018 12:22:27 AM
    //   Apr 17, 2018 09:32:00
    for fmt in &["%b %d, %Y %r", "%b %d, %Y %T"] {
        match NaiveDateTime::parse_from_str(input, fmt) {
            Ok(naive) => {
                let local: DateTime<Local> = Local.timestamp(naive.timestamp(), 0);
                return Ok(local.with_timezone(&Utc))
            }
            Err(err) => eprintln!("parse_from_str error: {}", err),
        }
    }
    Err(event::Error::ParseError)
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

    #[test]
    fn test_json_with_tag_raw_mode() {
        let raw = r##"{"deviceId":"854af65f-13db-4082-b07e-89129690d275","eventId":"449b03cc-171a-46b2-b367-310f1af81d21","tags":[{"accelX":-0.004,"accelY":0.112,"accelZ":1.004,"defaultBackground":1,"favorite":true,"gatewayUrl":"","humidity":31.5,"id":"D7:58:D2:87:08:F8","name":"Devitagi","pressure":1013.14,"rawDataBlob":{"blob":[4,52,23,0,-56,100]},"rssi":-32,"temperature":24.01,"updateAt":"Apr 16, 2018 21:11:27","voltage":3.097},{"accelX":0.936,"accelY":0.26,"accelZ":-0.204,"defaultBackground":6,"favorite":true,"gatewayUrl":"","humidity":70.0,"id":"D1:D8:2A:09:D6:C1","name":"Humidori","pressure":1013.0,"rawDataBlob":{"blob":[4,-116,22,0,-56,100]},"rssi":-68,"temperature":22.0,"updateAt":"Apr 16, 2018 21:11:27","url":"https://ruu.vi/#BIwWAMhkT","voltage":3.193}],"time":"Apr 16, 2018 21:11:27"}"##;
        let event: Tags = serde_json::from_str(raw).expect("serde_json::from_str");
        assert_eq!(event.time, "Apr 16, 2018 21:11:27");
    }

    #[test]
    fn test_json_to_event1() {
        let raw = r##"{"deviceId":"854af65f-13db-4082-b07e-89129690d275","eventId":"520a341b-49e7-49e6-b908-05a27da7d6ac","tags":[{"accelX":0.936,"accelY":0.26,"accelZ":-0.204,"defaultBackground":6,"favorite":true,"gatewayUrl":"","humidity":68.0,"id":"D1:D8:2A:09:D6:C1","name":"Humidori","pressure":1017.0,"rawDataBlob":{"blob":[4,-120,22,0,-55,-12]},"rssi":-74,"temperature":22.0,"updateAt":"Apr 15, 2018 21:16:17","url":"https://ruu.vi/#BIgWAMn0T","voltage":3.193},{"accelX":0.0,"accelY":0.0,"accelZ":0.0,"defaultBackground":1,"favorite":true,"gatewayUrl":"http://192.168.1.4:1337/","humidity":38.0,"id":"D7:58:D2:87:08:F8","name":"Devitagi","pressure":1017.0,"rawDataBlob":{"blob":[4,76,24,0,-55,-12]},"rssi":-56,"temperature":24.0,"updateAt":"Apr 15, 2018 21:16:17","url":"https://ruu.vi/#BEwYAMn0r","voltage":0.0}],"time":"Apr 15, 2018 21:16:17"}"##;
        let event: Vec<Event> = Tags::from_json(&raw).expect("from_json");
        assert_eq!(event.len(), 2);

    }

    #[test]
    fn test_json_to_event2() {
        let raw = r##"{"deviceId":"854af65f-13db-4082-b07e-89129690d275","eventId":"9e6329dd-06eb-474c-9d1d-9b4373704a6d","tag":{"accelX":0.0,"accelY":0.0,"accelZ":0.0,"defaultBackground":1,"favorite":true,"gatewayUrl":"http://192.168.1.4:1337/","humidity":22.0,"id":"D7:58:D2:87:08:F8","name":"Devitagi","pressure":996.0,"rawDataBlob":{"blob":[4,44,20,0,-63,-64]},"rssi":-57,"temperature":20.0,"updateAt":"Apr 14, 2018 12:22:27 AM","url":"https://ruu.vi/#BCwUAMHAr","voltage":0.0},"time":"Apr 14, 2018 12:22:27 AM"}"##;
        let event: Vec<Event> = Tags::from_json(&raw).expect("from_json");
        assert_eq!(event.len(), 1);
    }
}
