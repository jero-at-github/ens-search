use chrono::{TimeZone, Utc};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str;
use std::{thread, time};
use structs::*;

use crate::sha_util::sha3_hex;

const MAX_ENS_QUERY: usize = 100;
const ENS_URL: &str = "https://api.thegraph.com/subgraphs/name/ensdomains/ens";
const REQUEST_DELAY: u64 = 1000;

mod sha_util;
mod structs;
mod tests;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub async fn request_ens_data(ht: HashMap<String, String>) -> Result<EnsResponse, Box<dyn Error>> {
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

    let q = EnsQuery {
        query: q,
        variables: EnsQueryVariables {
            ids: ht_ids.clone(),
        },
    };

    // HTTP request
    let client = reqwest::Client::new();
    let res = client
        .post(ENS_URL)
        .json(&q)
        .header("content-type", "application/json")
        .send()
        .await?;
    let response = res.json::<EnsResponse>().await;

    match response {
        Ok(r) => return Ok(r),
        Err(e) => {
            println!("Problem calling API: {:?}", e);
            println!("Http Body: {:#?}", q);
            return Err(e.into());
        }
    };
}

fn process_ens_data(
    response: EnsResponse,
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

async fn request_ens_batch(
    ht: &mut HashMap<String, String>,
    total_processed: &mut usize,
    unregistered_domains: &mut Vec<String>,
    expired_domains: &mut Vec<String>,
) {
    let r = request_ens_data(ht.clone()).await;
    if let Ok(response_data) = r {
        let p = process_ens_data(response_data, ht.clone());
        let mut p_data = p.unwrap();
        unregistered_domains.append(&mut p_data.0);
        expired_domains.append(&mut p_data.1);

        thread::sleep(time::Duration::from_millis(REQUEST_DELAY));
        *total_processed += ht.clone().len();
        println!("Processed {} domains", total_processed);
        ht.clear();
    }
}

pub async fn process_file(file_name: String) {
    if let Ok(lines) = read_lines(file_name) {
        let mut ht: HashMap<String, String> = HashMap::new();
        let mut unregistered_domains: Vec<String> = vec![];
        let mut expired_domains: Vec<String> = vec![];
        let mut total_processed = 0;

        // let progress_ptg = lines.size_hint().1.unwrap() / 10;

        for (idx, line) in lines.enumerate() {
            if let Ok(domain_name) = line {
                let domain_name_norm = domain_name.to_lowercase();
                let domain_hash = sha3_hex(domain_name_norm.clone());
                ht.insert(domain_hash, domain_name_norm.clone());

                if idx != 0 && (idx + 1) % MAX_ENS_QUERY == 0 {
                    request_ens_batch(
                        &mut ht,
                        &mut total_processed,
                        &mut unregistered_domains,
                        &mut expired_domains,
                    )
                    .await;
                }
            }
        }

        if !ht.is_empty() {
            request_ens_batch(
                &mut ht,
                &mut total_processed,
                &mut unregistered_domains,
                &mut expired_domains,
            )
            .await;
        }

        println!("Not registered domains: {:#?}", unregistered_domains);
        println!("Expired domains: {:#?}", expired_domains);
    }
}
