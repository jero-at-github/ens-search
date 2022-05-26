use crate::{structs::*, ENS_URL, REQUEST_DELAY};
use chrono::{TimeZone, Utc};
use std::collections::HashMap;
use std::error::Error;
use std::{thread, time};

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

pub async fn request_ens_batch(
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
