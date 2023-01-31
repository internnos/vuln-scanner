use serde::Deserialize;

mod error;
mod subdomains;

use error::Error;
use subdomains::{get_request, postprocess_request};




#[tokio::main]
async fn main() -> Result<(), Error> {
    let target_domain = "kerkour.com";
    let json = get_request(target_domain).await?;
    let result = postprocess_request(json, target_domain);
    println!("{:?}", result);
    
    Ok(())
}