use crate::{Error, model::{CrtShEntry, Subdomain}};
use std::{collections::HashSet, time::Duration};
use futures::StreamExt;

use trust_dns_resolver::{
    config::{ResolverConfig, ResolverOpts},
    name_server::{GenericConnection, GenericConnectionProvider, TokioRuntime},
    AsyncResolver, proto::rr::domain,
};

pub async fn get_request(http_client: &reqwest::Client, target_domain: &str) -> Result<Vec<CrtShEntry>, Error> {
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
    // TODO: use keep alive connection pooling using Client as described https://docs.rs/reqwest/latest/reqwest/blocking/
    let response = http_client.get(endpoint).send().await?.json().await?;
    Ok(response)

}


pub async fn process_request(json_response: Vec<CrtShEntry>, target_domain: &str) -> Result<Vec<Subdomain>, Error> {
    let mut subdomains:HashSet<String> = json_response
    .into_iter()
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
    .into_iter()
    .map(|domain| {
        Subdomain {
            domain: domain.to_string(),
            open_ports: Vec::new()
        }
    })
    .collect();



    let subdomains: Vec<Subdomain> = futures::stream::iter(subdomains)
    .map(|subdomain| async move {
        let is_resolvable = resolves(&subdomain).await;
        if is_resolvable {
            Subdomain {
                domain: subdomain.domain,
                open_ports: Vec::new()
            }
            // print!("{} is resolvable)", subdomain.domain);
        } else {
            subdomain
        }
    })
    .buffer_unordered(100)
    .collect()
    .await;
    Ok(subdomains)



}


pub async fn resolves(domain: &Subdomain) -> bool {
    let mut dns_resolver_opts = ResolverOpts::default();
    dns_resolver_opts.timeout = Duration::from_secs(4);

    let dns_resolver = AsyncResolver::tokio(
        ResolverConfig::default(),
        dns_resolver_opts,
    )
    .expect("subdomain resolver: building DNS client");
    dns_resolver.lookup_ip(domain.domain.as_str()).await.is_ok()
}
