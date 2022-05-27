use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::io::{stdout, Write as IoWrite};
use std::path::Path;
use std::str;

use crossterm::{cursor, ExecutableCommand};
use structs::ProcessResult;

use crate::ens_util::request_ens_batch;
use crate::sha_util::sha3_hex;

pub const MAX_ENS_QUERY: usize = 100;
pub const ENS_URL: &str = "https://api.thegraph.com/subgraphs/name/ensdomains/ens";
pub const REQUEST_DELAY: u64 = 1000;

mod ens_util;
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

pub async fn process_file(file_name: String) {
    let mut process_result: ProcessResult = ProcessResult {
        unregistered_domains: vec![],
        expired_domains: vec![],
    };

    let mut file_result = File::create("./result.txt").unwrap();
    let mut file_errors = File::create("./errors.txt").unwrap();

    // Read lines from file
    if let Ok(lines) = read_lines(file_name) {
        let mut ht: HashMap<String, String> = HashMap::new();
        let mut total_processed = 0;

        // TODO:
        // let progress_ptg = lines.size_hint().1.unwrap() / 10;
        stdout().execute(cursor::SavePosition).unwrap();

        // Loop through lines
        for (idx, line) in lines.enumerate() {
            if let Ok(domain_name) = line {
                // Populate hashmap with hash-domain name pairs
                let domain_name_norm = domain_name.to_lowercase().trim().to_string();
                let domain_hash = sha3_hex(domain_name_norm.clone());
                ht.insert(domain_hash, domain_name_norm.clone());

                if idx != 0 && (idx + 1) % MAX_ENS_QUERY == 0 {
                    // Request API in batch
                    request_ens_batch(
                        &mut ht,
                        &mut total_processed,
                        &mut process_result,
                        &mut file_errors,
                    )
                    .await;
                }
            }
        }

        if !ht.is_empty() {
            // Request API in batch
            request_ens_batch(
                &mut ht,
                &mut total_processed,
                &mut process_result,
                &mut file_errors,
            )
            .await;
        }

        // sort expired domains by expiration date
        process_result
            .expired_domains
            .sort_by(|a, b| a.expiration_date.cmp(&b.expiration_date));
    }

    // Write results into file
    write!(file_result, "{:#?}", process_result).unwrap();
}
