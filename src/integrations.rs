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
        let result:Vec<model::Subdomain> = process_request(response, target_domain).unwrap()
        .into_par_iter()
        .map(|subdomain| scan_ports(subdomain))
        .collect();

        println!("{:?}", result);
    });
}



