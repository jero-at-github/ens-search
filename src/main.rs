// Reference:
// https://docs.ens.domains/contract-api-reference/name-processing
// https://eips.ethereum.org/EIPS/eip-137

use ens_domains::{display_vec, namehash};

mod tests;
fn main() {
    let r = namehash("eth".into());
    println!("{}", display_vec(r));
}
