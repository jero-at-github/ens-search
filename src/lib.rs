use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str;

const ENS_URL: &str = "https://api.thegraph.com/subgraphs/name/ensdomains/ens";

#[derive(Debug, Serialize, Deserialize)]
pub struct EnsQueryVariables {
    pub ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnsQuery {
    pub query: String,
    pub variables: EnsQueryVariables,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnsDomain {
    pub name: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct EnsRegistrations {
    pub id: String,
    pub registrationDate: String,
    pub expiryDate: String,
    pub domain: EnsDomain,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnsData {
    pub registrations: Vec<EnsRegistrations>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnsResponse {
    pub data: EnsData,
}

fn sha_vec_to_hex(vec: Vec<u8>) -> String {
    let result: String = vec
        .iter()
        .map(|x| format!("{:02x?}", x))
        .collect::<Vec<String>>()
        .join("");

    ["0x".into(), result].concat()
}

fn sha3(name: &[u8]) -> Vec<u8> {
    let mut hasher = Keccak256::new();
    hasher.update(name);
    let hasher_fin = hasher.finalize();
    let result: &[u8] = hasher_fin.as_slice();
    result.to_vec()
}

pub fn sha3_hex(name: String) -> String {
    sha_vec_to_hex(sha3(name.as_bytes()))
}

fn namehash(name: String) -> Vec<u8> {
    if name.is_empty() {
        [0; 32].to_vec()
    } else {
        let split_vec: Vec<&str> = name.split('.').collect();
        let label = split_vec.get(0).unwrap().to_string();
        let remainder: String = split_vec.get(1).or(Some(&"")).unwrap().to_string();

        sha3(
            [namehash(remainder), sha3(label.as_bytes())]
                .concat()
                .as_slice(),
        )
    }
}

pub fn namehash_hex(name: String) -> String {
    sha_vec_to_hex(namehash(name))
}

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
