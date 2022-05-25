// Reference:
// https://docs.ens.domains/contract-api-reference/name-processing
// https://eips.ethereum.org/EIPS/eip-137

use ens_domains::{read_lines, sha3_hex};
use std::error::Error;

use crate::structs::{Query, QueryVariables, Response};

mod structs;
mod tests;

async fn request_data(labelhash_ids: Vec<String>) -> Result<(), Box<dyn Error>> {
    // let mut map = HashMap::new();
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

    // let ids: Vec<String> = vec![
    //     "0xaaeb548a149a78dfc2f2d4e6838f5cba9f65c55bcefa06258e77335cf32b4452".into(),
    //     "0x41b1a0649752af1b28b3dc29a1556eee781e4a4c3a1f7f53f90fa834de098c4d".into(),
    // ];
    let v = QueryVariables { ids: labelhash_ids };

    let q = Query {
        query: q,
        variables: v,
    };

    // let json = serde_json::to_string(&q).unwrap();

    let client = reqwest::Client::new();
    let res = client
        .post("https://api.thegraph.com/subgraphs/name/ensdomains/ens")
        .json(&q)
        .header("content-type", "application/json")
        .send()
        .await?;

    let js = res.json::<Response>().await?;
    //println!("{:#?}", js);
    // println!("{:#?}", js.data.registrations.get(0).unwrap().id);

    let expiration_dates: Vec<String> = js
        .data
        .registrations
        .iter()
        .map(|r| r.expiryDate.clone())
        .collect();

    println!("{:#?}", expiration_dates);

    // let person: Person = serde_json::from_str(&js.data)?;
    // println!("{:#?}", person);

    Ok(())
}

async fn process_file() {
    if let Ok(lines) = read_lines("./sample/3letters.txt") {
        let mut labelhash_ids: Vec<String> = Vec::new();

        for (idx, line) in lines.enumerate() {
            if let Ok(domain_name) = line {
                labelhash_ids.push(sha3_hex(domain_name.to_lowercase()));
                if idx != 0 && idx % 100 == 0 {
                    request_data(labelhash_ids.clone()).await.unwrap();
                    labelhash_ids.clear();
                    break;
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    process_file().await;
}
