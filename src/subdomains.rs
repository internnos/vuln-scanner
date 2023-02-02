use crate::{Error, model::{CrtShEntry, Subdomain}};
use std::{collections::HashSet};
use reqwest::blocking::get;

use trust_dns_resolver::{
    config::{ResolverConfig, ResolverOpts},
    Resolver,
};

pub fn get_request(target_domain: &str) -> Result<Vec<CrtShEntry>, Error> {
    // response would be in the following format
    // [
    //     {'issuer_ca_id': 157938, 'issuer_name': 'C=US, O="Cloudflare, Inc.", CN=Cloudflare Inc ECC CA-3', 'common_name': 'kerkour.com', 'name_value': 'kerkour.com', 'id': 8491193860, 'entry_timestamp': '2023-01-25T02:58:20.746', 'not_before': '2023-01-25T00:00:00', 'not_after': '2024-01-25T23:59:59', 'serial_number': '0314749c7e5ad6c3814b4dd0d9c4df9a'},
    //     {'issuer_ca_id': 183267, 'issuer_name': "C=US, O=Let's Encrypt, CN=R3", 'common_name': 'social.kerkour.com', 'name_value': 'social.kerkour.com', 'id': 8500240922, 'entry_timestamp': '2023-01-26T06:15:37.143', 'not_before': '2023-01-26T05:15:37', 'not_after': '2023-04-26T05:15:36', 'serial_number': '03e009552edd08c4383a354a66413d702bc7'}
    // ]
    // the goal is to extract the "name_value" field
    // however, there can be multiple value in "name_value" field which will be separated by "\n" token
    // {'issuer_ca_id': 180753, 'issuer_name': 'C=US, O=Google Trust Services LLC, CN=GTS CA 1P5', 'common_name': '*.kerkour.com', 'name_value': '*.kerkour.com\nkerkour.com', 'id': 8479263334, 'entry_timestamp': '2023-01-23T04:06:27.34', 'not_before': '2023-01-23T03:06:26', 'not_after': '2023-04-23T03:06:25', 'serial_number': '6b6fd1b09d2cd76a0ee5dfd5fc6c8e90'}
    // Ok(entries)
    let endpoint = format!("https://crt.sh/?q=%25.{target_domain}&output=json");
    let entries = get(endpoint)?;
    let response: Vec<CrtShEntry> = entries.json()?;
    Ok(response)
    
}


pub fn postprocess_request(json_response: Vec<CrtShEntry>, target_domain: &str) -> Result<Vec<Subdomain>, Error> {
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
    
    let subdomains: Vec<Subdomain> = subdomains
    .iter()
    .map(|domain| {
        Subdomain {
            domain: domain.to_string(),
            open_ports: Vec::new()
        }
    })
    .filter(resolves)
    .collect();
    Ok(subdomains)

}


pub fn resolves(domain: &Subdomain) -> bool {
    let dns_resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).expect("subdomain resolver: building DNS client");
    dns_resolver.lookup_ip(domain.domain.as_str()).is_ok()
}
