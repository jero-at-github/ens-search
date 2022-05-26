// Reference:
// https://docs.ens.domains/contract-api-reference/name-processing
// https://eips.ethereum.org/EIPS/eip-137

use chrono::{TimeZone, Utc};
use ens_domains::{read_lines, sha3_hex};
use std::collections::HashMap;
use std::error::Error;
use std::{thread, time};

use crate::structs::{Query, QueryVariables, Response};

mod structs;
mod tests;

async fn request_data(ht: HashMap<String, String>) -> Result<Response, Box<dyn Error>> {
    // Extract ids from hashmap
    let ht_ids = ht
        .iter()
        .map(|ht| String::from(ht.0))
        .collect::<Vec<String>>();

    // Build GraphQL query
    let q = r#"
    query getName($ids: [ID!]) {
        registrations(where: { id_in: $ids }) {
            id
            labelName
            expiryDate
            registrationDate
            domain {
                name
            }
        }
    }"#
    .into();

    let q = Query {
        query: q,
        variables: QueryVariables {
            ids: ht_ids.clone(),
        },
    };

    // HTTP request
    let client = reqwest::Client::new();
    let res = client
        .post("https://api.thegraph.com/subgraphs/name/ensdomains/ens")
        .json(&q)
        .header("content-type", "application/json")
        .send()
        .await?;
    let response = res.json::<Response>().await;
    match response {
        Ok(r) => return Ok(r),
        Err(e) => {
            println!("Http Body: {:#?}", q);
            panic!("Problem calling API: {:?}", e);
        }
    };
}

fn process_data(
    response: Response,
    ht: HashMap<String, String>,
) -> Result<(Vec<String>, Vec<String>), Box<dyn Error>> {
    // Collect response domain ids and expiration dates
    let response_ids: Vec<String> = response
        .data
        .registrations
        .iter()
        .map(|r| r.id.clone())
        .collect();

    let response_exp_dates: Vec<(String, String)> = response
        .data
        .registrations
        .iter()
        .map(|r| (r.id.clone(), r.expiryDate.clone()))
        .collect();

    let mut not_registered_domains: Vec<String> = vec![];
    let mut expired_domains: Vec<String> = vec![];

    let mut ht_iter = ht.iter();
    let mut ht_value = ht_iter.next();

    // Check unregistered domains
    while ht_value.is_some() {
        let hash = ht_value.unwrap().0;
        let domain_name = ht_value.unwrap().1;

        // Check if the rquest response contains the hash
        if !response_ids.contains(hash) {
            not_registered_domains.push(domain_name.clone());
        }

        ht_value = ht_iter.next();
    }

    // Check expired domains
    let mut exp_dates_iter = response_exp_dates.iter();
    let mut exp_dates_value = exp_dates_iter.next();
    while exp_dates_value.is_some() {
        let hash = &exp_dates_value.unwrap().0;
        let timestamp = &exp_dates_value.unwrap().1;
        let dt = Utc.timestamp(timestamp.parse::<i64>().unwrap(), 0);

        if dt.lt(&Utc::now()) {
            let domain_name = ht.get(hash).unwrap();
            expired_domains.push(domain_name.clone());
        }
        exp_dates_value = exp_dates_iter.next();
    }

    Ok((not_registered_domains, expired_domains))
}

async fn process_file() {
    if let Ok(lines) = read_lines("./sample/3letters.txt") {
        let mut ht: HashMap<String, String> = HashMap::new();
        let mut unregistered_domains: Vec<String> = vec![];
        let mut expired_domains: Vec<String> = vec![];
        // let progress_ptg = lines.size_hint().1.unwrap() / 10;

        let mut total_processed = 0;
        for (idx, line) in lines.enumerate() {
            if let Ok(domain_name) = line {
                let domain_name_norm = domain_name.to_lowercase();
                let domain_hash = sha3_hex(domain_name_norm.clone());
                ht.insert(domain_hash, domain_name_norm.clone());

                if idx != 0 && (idx + 1) % 100 == 0 {
                    // request ENS API
                    let r = request_data(ht.clone())
                        .await
                        .expect("Error requesting API");

                    let p = process_data(r, ht.clone());
                    let p_data = p.unwrap();
                    unregistered_domains.append(&mut p_data.clone().0);
                    expired_domains.append(&mut p_data.clone().1);

                    ht.clear();
                    thread::sleep(time::Duration::from_secs(1));
                    total_processed += 100;
                    println!("Processed {} domains", total_processed);
                }
            }
        }

        if !ht.is_empty() {
            // request ENS API
            let r = request_data(ht.clone()).await.unwrap();
            let p = process_data(r, ht.clone());
            let p_data = p.unwrap();
            unregistered_domains.append(&mut p_data.clone().0);
            expired_domains.append(&mut p_data.clone().1);

            println!("Processed {} domains", total_processed + ht.clone().len());
        }

        ht.clear();

        println!("Not registered domains: {:#?}", unregistered_domains);
        println!("Expired domains: {:#?}", expired_domains);
    }
}

#[tokio::main]
async fn main() {
    process_file().await;
}
