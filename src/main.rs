// Reference:
// https://docs.ens.domains/contract-api-reference/name-processing
// https://eips.ethereum.org/EIPS/eip-137

use ens_domains::process_file;

#[tokio::main]
async fn main() {
    process_file("./sample/3letters.txt".into()).await;
}
