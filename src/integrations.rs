use serde::Deserialize;

mod error;
mod subdomains;
mod model;
mod ports;

use error::Error;
use subdomains::{get_request, process_request};

use crate::ports::scan_ports;

fn run_multithread(){
    let client = reqwest::blocking::Client::new();

    let pool = rayon::ThreadPoolBuilder::new()
    .num_threads(256)
    .build()?;

    pool.install(|| {
        let target_domain = "kerkour.com";
        let response = get_request(&client, target_domain).unwrap();
        let subdomains:Vec<model::Subdomain> = process_request(response, target_domain).unwrap()
        .into_par_iter()
        .map(|subdomain| scan_ports(subdomain))
        .collect();

        println!("{:?}", subdomains);
    });
}

async fn run_async(){
    let target_domain = "kerkour.com";
    let http_timeout = Duration::from_secs(10);
    let client = reqwest::blocking::Client::new();
    let http_client = reqwest::blocking::Client::builder().timeout(http_timeout).build()?;

    let ports_concurrency = 200;
    let subdomains_concurrency = 100;
    let scan_start = Instant::now();

    let response = get_request(&client, target_domain).unwrap();
    let subdomains = subdomains::process_request(response, target_domain).await?;
}





