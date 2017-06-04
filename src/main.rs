extern crate hyper;
extern crate hyper_native_tls;

// Note: this matters (it affects serde_derive)
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use hyper::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use std::io::Read;

#[derive(Deserialize)]
struct Status {
    status: String,
}

fn main() {
    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    let client = Client::with_connector(connector);

    let mut res = client
        .get("https://status.github.com/api/status.json")
        .send()
        .unwrap();
    assert_eq!(res.status, hyper::Ok);
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    let current: Status = serde_json::from_str(&body).unwrap();
    if current.status == "good" {
        println!("\u{2713}");
    } else {
        println!("\u{2718}");
    }
}
