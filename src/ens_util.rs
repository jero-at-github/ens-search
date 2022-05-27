use crate::{structs::*, ENS_URL, REQUEST_DELAY};
use chrono::{TimeZone, Utc};
use crossterm::{cursor, ExecutableCommand};
use reqwest::Response;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, stdout, Write as IoWrite};
use std::{thread, time};

fn process_ens_data(
    response: EnsResponse,
    ht: HashMap<String, String>,
) -> Result<ProcessResult, Box<dyn Error>> {
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

    let mut unregistered_domains: Vec<String> = vec![];
    let mut expired_domains: Vec<ExpiredDomain> = vec![];

    let mut ht_iter = ht.iter();
    let mut ht_value = ht_iter.next();

    // Check unregistered domains
    while ht_value.is_some() {
        let hash = ht_value.unwrap().0;
        let domain_name = ht_value.unwrap().1;

        // Check if the rquest response contains the hash
        if !response_ids.contains(hash) {
            unregistered_domains.push(domain_name.clone());
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
            expired_domains.push(ExpiredDomain {
                domain_name: domain_name.clone(),
                expiration_date: dt,
            });
        }
        exp_dates_value = exp_dates_iter.next();
    }

    Ok(ProcessResult {
        unregistered_domains,
        expired_domains,
    })
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

    let query = EnsQuery {
        query: q,
        variables: EnsQueryVariables {
            ids: ht_ids.clone(),
        },
    };

    // HTTP request
    let client = reqwest::Client::new();
    let res: Response = client
        .post(ENS_URL)
        .json(&query)
        .header("content-type", "application/json")
        .send()
        .await?;
    let status = res.status();
    if status.is_success() {
        let response = res.json::<EnsResponse>().await;

        match response {
            Ok(r) => return Ok(r),
            Err(e) => {
                let err_msg = format!(
                    "
                Problem calling API: {:?}
                Http Body: {:#?}
                ",
                    e, query
                );
                return Err(err_msg.into());
            }
        };
    } else {
        let err_msg = format!(
            "
        Problem calling API, status code: {:#?}
        Http Body: {:#?}
        ",
            status.as_str(),
            query
        );
        return Err(err_msg.into());
    }
}

pub async fn request_ens_batch(
    ht: &mut HashMap<String, String>,
    total_processed: &mut usize,
    process_result: &mut ProcessResult,
    file_errors: &mut File,
) {
    // Request API
    let r = request_ens_data(ht.clone()).await;

    if let Ok(response_data) = r {
        // Filter data from response
        let mut p_data = process_ens_data(response_data, ht.clone()).unwrap();

        // Append filtered data to result
        process_result
            .unregistered_domains
            .append(&mut p_data.unregistered_domains);
        process_result
            .expired_domains
            .append(&mut p_data.expired_domains);

        thread::sleep(time::Duration::from_millis(REQUEST_DELAY));
        *total_processed += ht.clone().len();
        print!("Processed domains: {} ", total_processed);
        io::stdout().flush().unwrap();
        stdout().execute(cursor::RestorePosition).unwrap();
        ht.clear();
    } else {
        // Write results into file
        write!(file_errors, "{:#?}", r.err().unwrap()).unwrap();
    }
}
