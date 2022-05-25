// Reference:
// https://docs.ens.domains/contract-api-reference/name-processing
// https://eips.ethereum.org/EIPS/eip-137

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use ens_domains::domain_to_hash;

mod tests;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("../../sample/3letters.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(name) = line {
                let domain_name = [name, ".eth".into()].concat();
                println!("{}", domain_name);
                println!("{}", domain_to_hash(domain_name));
            }
        }
    }
}
