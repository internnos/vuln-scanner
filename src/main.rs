use serde::Deserialize;
use std::collections::HashSet;
use thiserror::Error;


#[derive(Debug, Deserialize, Clone)]
pub struct CrtShEntry {
    pub name_value: String,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Usage: tricoder <kerkour.com>")]
    CliUsage,
    #[error("Reqwest Error")]
    ReqwestError(#[from] reqwest::Error)
}

async fn get_request(target_domain: &str) -> Result<Vec<CrtShEntry>, Error> {
    let endpoint = format!("https://crt.sh/?q=%25.{target_domain}&output=json");
    let res = reqwest::get(endpoint).await?;
    let json:Vec<CrtShEntry> = res.json().await?;
    // response would be in the following format
    // [
    //     {'issuer_ca_id': 157938, 'issuer_name': 'C=US, O="Cloudflare, Inc.", CN=Cloudflare Inc ECC CA-3', 'common_name': 'kerkour.com', 'name_value': 'kerkour.com', 'id': 8491193860, 'entry_timestamp': '2023-01-25T02:58:20.746', 'not_before': '2023-01-25T00:00:00', 'not_after': '2024-01-25T23:59:59', 'serial_number': '0314749c7e5ad6c3814b4dd0d9c4df9a'},
    //     {'issuer_ca_id': 183267, 'issuer_name': "C=US, O=Let's Encrypt, CN=R3", 'common_name': 'social.kerkour.com', 'name_value': 'social.kerkour.com', 'id': 8500240922, 'entry_timestamp': '2023-01-26T06:15:37.143', 'not_before': '2023-01-26T05:15:37', 'not_after': '2023-04-26T05:15:36', 'serial_number': '03e009552edd08c4383a354a66413d702bc7'}
    // ]
    // the goal is to extract the "name_value" field
    // however, there can be multiple value in "name_value" field which will be separated by "\n" token
    // {'issuer_ca_id': 180753, 'issuer_name': 'C=US, O=Google Trust Services LLC, CN=GTS CA 1P5', 'common_name': '*.kerkour.com', 'name_value': '*.kerkour.com\nkerkour.com', 'id': 8479263334, 'entry_timestamp': '2023-01-23T04:06:27.34', 'not_before': '2023-01-23T03:06:26', 'not_after': '2023-04-23T03:06:25', 'serial_number': '6b6fd1b09d2cd76a0ee5dfd5fc6c8e90'}
    Ok(json)
}


fn postprocess_request(json_response: Vec<CrtShEntry>, target_domain: &str) -> HashSet<String> {
    let mut subdomains:HashSet<String> = json_response
    .iter()
    .flat_map(|entry| {
        entry
        .name_value
        .split("\n")
        .map(|subdomain| subdomain.trim().to_string())
        .collect::<Vec<String>>() 
    })
    .filter(|subdomain| subdomain != target_domain)
    .filter(|subdomain| !subdomain.contains("*"))
    .collect();
    subdomains.insert(target_domain.to_string());
    subdomains
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let target_domain = "kerkour.com";
    let json = get_request(target_domain).await?;
    let result = postprocess_request(json, target_domain);
    println!("{:?}", result);
    
    Ok(())
}