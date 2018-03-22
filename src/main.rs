#[macro_use]
extern crate rouille;
#[macro_use]
extern crate serde_derive;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Tags {
    device_id: String,
    event_id: String,
    tag: Option<Tag>,
    tags: Option<Vec<Tag>>,
    time: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Tag {
    accel_x: f32,
    accel_y: f32,
    accel_z: f32,
    default_background: u32,
    favorite: bool,
    humidity: f32,
    id: String,
    name: Option<String>,
    pressure: f32,
    raw_data_blob: Blob,
    rssi: i32,
    temperature: f32,
    update_at: String,
    voltage: f32,
}

#[derive(Debug, Deserialize)]
struct Blob {
    blob: Vec<i32>,
}

fn main() {
    println!("Now listening on 10.10.20.11:1337");

    rouille::start_server("10.10.20.11:1337", move |request| {
        router!(request,
            (POST) (/) => {
                let tags: Tags = rouille::input::json_input(request).unwrap();
                println!("{:?}", tags);

                rouille::Response::text("lol")
            },
            _ => rouille::Response::empty_404()
        )
    });
}
