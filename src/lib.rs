use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str;

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
    if let Ok(lines) = read_lines(file_name) {
        let mut ht: HashMap<String, String> = HashMap::new();
        let mut unregistered_domains: Vec<String> = vec![];
        let mut expired_domains: Vec<String> = vec![];
        let mut total_processed = 0;

        // TODO:
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
