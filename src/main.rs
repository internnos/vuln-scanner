use serde::Deserialize;

mod error;
mod subdomains;
mod model;
mod ports;

use error::Error;
use subdomains::{get_request, process_request};

use crate::ports::scan_ports;

use rayon::prelude::*;

use std::time::Duration;
use std::time::Instant;


#[tokio::main]
async fn main() -> Result<(), Error> {
    let target_domain = "kerkour.com";
    let http_timeout = Duration::from_secs(10);
    let client = reqwest::Client::new();
    let http_client = reqwest::Client::builder().timeout(http_timeout).build();

    let ports_concurrency = 200;
    let subdomains_concurrency = 100;
    let scan_start = Instant::now();

    let response = get_request(&client, target_domain).await?;
    let subdomains = process_request(response, target_domain).await?;
    Ok(())
}
