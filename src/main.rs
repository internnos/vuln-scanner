use serde::Deserialize;

mod error;
mod subdomains;
mod model;
mod ports;

use error::Error;
use subdomains::{get_request, process_request};

use crate::ports::scan_ports;



fn main() -> Result<(), Error> {
    let target_domain = "kerkour.com";
    let response = get_request(target_domain)?;
    let result:Vec<model::Subdomain> = process_request(response, target_domain)?
    .into_iter()
    .map(|subdomain| scan_ports(subdomain))
    .collect();

    println!("{:?}", result);
    Ok(())
}
