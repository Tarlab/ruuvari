#[macro_use]
extern crate rouille;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[macro_use]
extern crate log;
extern crate env_logger;

use std::io::prelude::*;

mod ruuvistation;
mod beaconscanner;
mod event;

use std::net::SocketAddr;

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
                let event: Result<ruuvistation::Tags, _> = serde_json::from_str(&body);
                if let Ok(e) = event {
                    println!("Ruuvi Station: {:?}", e);
                }

                // Try Beacon Scanner
                let event: Result<beaconscanner::Beacons, _> = serde_json::from_str(&body);
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
