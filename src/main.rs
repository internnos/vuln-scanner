use serde::Deserialize;

mod error;
mod subdomains;
mod model;

use error::Error;
use subdomains::{get_request, postprocess_request};



fn main() -> Result<(), Error> {
    let target_domain = "kerkour.com";
    let json = get_request(target_domain);
    // let result = postprocess_request(json, target_domain);
    println!("{:?}", json);
    Ok(())
}
