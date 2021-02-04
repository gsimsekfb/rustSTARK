

### API

```
pub fn rescue_verify(
    proof_hex: &str,
    public_input_json: &str, 
    parameters_json: &str, 
    annotation_file_name: &str
) -> bool { .. }
```

**Example params:**  
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


