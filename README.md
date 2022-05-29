# ens-search
ENS domains bulk search tool

### What

ens-search is a CLI tool implemented in Rust for searching Web3 ENS domains: https://docs.ens.domains/
It supports unlimited bulk search through a text file used as input.

### How to install

- Install Rust: https://www.rust-lang.org/tools/install
- Compile the code: `cargo install`
- Check installation: `ens-search --help`
 
### How to use 
- Prepare a file (`foo.txt`) with the input. The file should contain one domain per line:
    ```
    this-domain-is-not-registered.eth
    this-one-neither.eth
    foo.eth
    ...
    ```
    Note: the `.eth` extension is optional, it will work the same with or without it.
- Execute the tool: `ens-search file {./myInput.txt`}
- Two files will be generated in the current folder:
. `result.txt` containing a JSON structure with information regarding non registered domains and expired domains: 
    ```
    ProcessResult {
        unregistered_domains: [
            "this-domain-is-not-registered",
            "this-one-neither",
        ],
        expired_domains: [
            ExpiredDomain {
                domain_name: "foo",
                expiration_date: 2022-03-13T15:18:59Z,
            },
        ],
    }
    ```
    . `errors.txt` containing any possible error appeared during the bulk search e.g. a specific HTTP request failed becuase non reponsive server.
