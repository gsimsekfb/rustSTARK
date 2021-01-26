


### How to use this library? 

Make these changes in your repo:  

.cargo/config.toml
```
[build]
rustflags = ["-C", "link-args=-no-pie"]
```

Cargo.toml
```
[dependencies]
rust_call_cpp = { git = "https://github.com/simsekgokhan/rust_call_cpp_so", branch = "main" }
```


main.rs

```
mod example_proof;
use rust_call_cpp::rescue_verify;

fn main() {
    println!("Hello, world!");    

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





### Info: How to call C++ from Rust using shared library method

```
Ubuntu 20.04

// 1. Edit libhello_cpp/hello.cpp as you like

// 2. Create C++ shared library for libhello_cpp/hello.cpp 
cd libhello_cpp
g++ -shared -fPIC -o libhello.so hello.cpp 

// 3. Put your dependency into build.rs
println!("cargo:rustc-link-search=native=/home/gsimsek/rust_call_cpp/libhello_cpp/");        
println!("cargo:rustc-link-lib=dylib=hello"); 
    
// 4. Run rust app
$ cargo r
Hello from C++ say_hello()
  // main.rs calling C++ function -
  // void say_hello(std::string const& str, int reps) -
  // from libhello_cpp/hello.cpp 

```

### Sources
https://crates.io/crates/cc  
https://www.youtube.com/watch?v=iCZkqDMJiF8  

