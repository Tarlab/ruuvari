#[macro_use]
extern crate rouille;
extern crate serde_json;

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate ruuvari;

use std::io::prelude::*;
use std::net::SocketAddr;

use ruuvari::{Event, ToRuuvariEvent};
use ruuvari::ruuvistation;
use ruuvari::beaconscanner;

fn main() {
    env_logger::init();
    let listen_on: SocketAddr = "0.0.0.0:1337".parse().expect("Parse listen_on address");

    println!("Now listening on {}", listen_on);

    rouille::start_server(listen_on, move |request| {
        router!(request,
            (POST) (/) => {
                let mut data = request.data().unwrap();
                let mut body = String::new();
                data.read_to_string(&mut body).unwrap();
                println!("Request body: {}", body);

                // Try Ruuvi Station
                let event: Result<Vec<Event>, _> = ruuvistation::Tags::from_json(&body);
                if let Ok(e) = event {
                    println!("Ruuvi Station: {:?}", e);
                }

                // Try Beacon Scanner
                let event: Result<Vec<Event>, _> = beaconscanner::Beacons::from_json(&body);
                if let Ok(e) = event {
                    println!("Beacon Scanner: {:?}", e);
                }

                println!();
                rouille::Response::text("lol")
            },
            _ => {
                debug!("Invalid request: {:?}", request);
                rouille::Response::empty_404()
            }
        )
    });
}
