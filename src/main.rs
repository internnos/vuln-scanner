use serde::Deserialize;

mod error;
mod subdomains;
mod model;

use error::Error;
use subdomains::{get_request, process_request};



fn main() -> Result<(), Error> {
    let target_domain = "kerkour.com";
    let response = get_request(target_domain)?;
    let result = process_request(response, target_domain);
    println!("{:?}", result);
    Ok(())
}
