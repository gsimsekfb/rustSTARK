
### INFO

This Rust library creates a basic wrapper/API for C++ library https://github.com/simsekgokhan/ethSTARK. 

Currently, using the quickest option which is linking with these ethSTARK static library files here:  
https://github.com/simsekgokhan/rustSTARK/tree/main/ethSTARK



### API

```
pub fn rescue_verify(
    proof_hex: &str,
    public_input_json: &str, 
    parameters_json: &str, 
    annotation_file_name: &str
) -> bool { .. }

// Note: prove() has the old signature - which accepts file paths
pub fn rescue_prove(
    parameter_file: &str,
    prover_config_file: &str,
    public_input_file: &str,
    private_input_file: &str,
    out_file: &str
) { .. }
```

**Example parameters:**  
https://github.com/simsekgokhan/rustSTARK/blob/main/src/example_proof.rs  
// Also in file format:  
https://github.com/simsekgokhan/rustSTARK/tree/main/example

### How to use this library? 

Make these changes in your repo:  

Cargo.toml  
```
[dependencies]
zkp = { git = "https://github.com/simsekgokhan/rustSTARK", branch = "main" }
```

main.rs  
```
use zkp::rescue_verify;
use zkp::example_proof;

fn main() {
    let result = rescue_verify(
        example_proof::PROOF_HEX, 
        example_proof::PUBLIC_INPUT,
        example_proof::PARAMETERS, 
        ""
    );
    println!("--- Rust: Proof verified: {:?}", result);
}
```

Expected output:

```
----------- unsafe C++ call   -----------

WARNING: Logging before InitGoogleLogging() is written to STDERR
I0126 13:57:38.424813  3635 rescue_verifier.cc:35] Proof verified successfully.

----------- end of unsafe C++ -----------

--- Rust: Proof verified: true
```


