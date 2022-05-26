// Reference:
// https://docs.ens.domains/contract-api-reference/name-processing
// https://eips.ethereum.org/EIPS/eip-137

use ens_domains::{read_lines, sha3_hex};
use std::collections::HashMap;
use std::error::Error;
use std::{thread, time};

use crate::structs::{Query, QueryVariables, Response};

mod structs;
mod tests;

async fn request_data(ht: HashMap<String, String>) -> Result<Vec<String>, Box<dyn Error>> {
    let ht_ids = ht
        .iter()
        .map(|ht| String::from(ht.0))
        .collect::<Vec<String>>();

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

    let v = QueryVariables {
        ids: ht_ids.clone(),
    };

    let q = Query {
        query: q,
        variables: v,
    };

    let client = reqwest::Client::new();
    let res = client
        .post("https://api.thegraph.com/subgraphs/name/ensdomains/ens")
        .json(&q)
        .header("content-type", "application/json")
        .send()
        .await?;

    let response = res.json::<Response>().await?;

    // Check for not registered domains
    let response_ids: Vec<String> = response
        .data
        .registrations
        .iter()
        .map(|r| r.id.clone())
        .collect();

    let mut not_registered_domains: Vec<String> = vec![];
    let mut ht_ids_iter = ht_ids.iter();
    let mut ht_id = ht_ids_iter.next();
    while ht_id.is_some() {
        let ht_id_value = ht_id.unwrap();
        if !response_ids.contains(ht_id_value) {
            let domain_name = ht.get(ht_id_value).unwrap();
            not_registered_domains.push(domain_name.clone());
        }
        ht_id = ht_ids_iter.next();
    }

    // let expiration_dates: Vec<String> = js
    //     .data
    //     .registrations
    //     .iter()
    //     .map(|r| r.expiryDate.clone())
    //     .collect();

    // let mut ex_dates_iter = expiration_dates.iter();
    // let mut timestamp = ex_dates_iter.next();
    // while timestamp.is_some() {
    //     let dt = Utc.timestamp(timestamp.unwrap().parse::<i64>().unwrap(), 0);
    //     println!("{:#?}", dt);
    //     timestamp = ex_dates_iter.next();
    // }

    Ok(not_registered_domains)
}

async fn process_file() {
    if let Ok(lines) = read_lines("./sample/3letters.txt") {
        let mut labelhash_ids: HashMap<String, String> = HashMap::new();
        let mut not_registered_domains: Vec<String> = vec![];
        // let progress_ptg = lines.size_hint().1.unwrap() / 10;

        let mut total_processed = 0;
        for (idx, line) in lines.enumerate() {
            if let Ok(domain_name) = line {
                let domain_name_norm = domain_name.to_lowercase();
                let domain_hash = sha3_hex(domain_name_norm.clone());
                labelhash_ids.insert(domain_hash, domain_name_norm.clone());

                if idx != 0 && (idx + 1) % 100 == 0 {
                    // request ENS API
                    let mut r = request_data(labelhash_ids.clone()).await.unwrap();
                    not_registered_domains.append(&mut r);

                    labelhash_ids.clear();
                    thread::sleep(time::Duration::from_secs(1));
                    total_processed += 1;
                    println!("Processed {} domains", total_processed);
                }
            }
        }

        if !labelhash_ids.is_empty() {
            let mut r = request_data(labelhash_ids.clone()).await.unwrap();
            not_registered_domains.append(&mut r);
            labelhash_ids.clear();
            println!(
                "Processed {} domains",
                total_processed + labelhash_ids.len()
            );
        }

        println!("Not registered domains: {:#?}", not_registered_domains);
    }
}

#[tokio::main]
async fn main() {
    process_file().await;
}
