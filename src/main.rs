use reqwest::Error;
use serde::Deserialize;
use std::collections::HashSet;


#[derive(Debug, Deserialize, Clone)]
pub struct CrtShEntry {
    pub name_value: String,
}

async fn get_request(target_domain: &str) -> Result<Vec<CrtShEntry>, Error> {
    let endpoint = format!("https://crt.sh/?q=%25.{target_domain}&output=json");
    let res = reqwest::get(endpoint).await?;
    // read to json
    let json:Vec<CrtShEntry> = res.json().await?;
    Ok(json)
}

async fn postprocess_request(json: Vec<CrtShEntry>, target_domain: &str) -> HashSet<String> {
    let mut subdomains: HashSet<String> = json
    .iter()
    .flat_map(|entry| {
        entry
            .name_value
            .split('\n')
            .map(|subdomain| subdomain.trim().to_string())
            .collect::<Vec<String>>()
    })
    .filter(|subdomain: &String| subdomain != target_domain)
    .filter(|subdomain: &String| !subdomain.contains('*'))
    .collect(); 
    subdomains.insert("wtf".to_string());
    subdomains
}


#[tokio::main]
async fn main() -> Result<(), Error> {
    // let client = Client::new();
    let target_domain = "kerkour.com";
    // let text = get_subdomain(target_domain).await?;
    let json = get_request(target_domain).await?;
    
    // print json
    
    println!("{:?}", json);


    // let json: Vec<> = res.json().await?;

    // let result = serde_json::from_str::<Obj>(&json).unwrap();
    // for i in result.items {
    //     println!("{:#?}", i);
    // }
    // println!("{:?}", result);
    
    Ok(())
}