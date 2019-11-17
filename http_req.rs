// cargo-deps: hyper = "0.12.35", tokio-core = "0.1.17", futures = "0.1.29"

extern crate futures;
extern crate hyper;
extern crate tokio_core;

use futures::{Future};
use hyper::{Client, Uri};
use tokio_core::reactor::Core;

fn main() {
    // Core is the Tokio event loop used for making a non-blocking request
    let mut core = Core::new().unwrap();

    let client = Client::new();

    let url : Uri = "http://httpbin.org/response-headers?foo=bar".parse().unwrap();
    assert_eq!(url.query(), Some("foo=bar"));

    let _request_result = core.run(client
        .get(url)
        .map(|res| {
            println!("Response: {}", res.status());
        })
        .map_err(|err| {
            println!("Error: {}", err);
       })
    );

}