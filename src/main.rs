// Reference:
// https://docs.ens.domains/contract-api-reference/name-processing
// https://eips.ethereum.org/EIPS/eip-137

use ens_domains::{process_ens_data, read_lines, request_ens_data, sha3_hex};
use std::collections::HashMap;
use std::{thread, time};

const MAX_ENS_QUERY: usize = 100;

#[tokio::main]
async fn main() {
    process_file("./sample/3letters.txt".into()).await;
}

async fn request_ens_batch(
    ht: &mut HashMap<String, String>,
    total_processed: &mut usize,
    unregistered_domains: &mut Vec<String>,
    expired_domains: &mut Vec<String>,
) {
    let r = request_ens_data(ht.clone()).await;
    match r {
        Ok(response_data) => {
            let p = process_ens_data(response_data, ht.clone());
            let mut p_data = p.unwrap();
            unregistered_domains.append(&mut p_data.0);
            expired_domains.append(&mut p_data.1);

            thread::sleep(time::Duration::from_secs(1));
            *total_processed += ht.clone().len();
            println!("Processed {} domains", total_processed);
            ht.clear();
        }
        _ => {}
    };
}

async fn process_file(file_name: String) {
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
