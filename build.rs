/*
    https://crates.io/crates/cc
    A library to compile C/C++/assembly into a Rust library/application.
    > Documentation:
    A simple library meant to be used as a build dependency with Cargo 
    packages in order to build a set of C/C++ files into a static archive. 
    This crate calls out to the most relevant compiler for a platform, 
    for example using cl on MSVC.
*/

#[allow(non_snake_case)]

fn main() {
    cc::Build::new()
        .cpp(true)
        .compiler("/usr/bin/clang++")
        .file("cpp_bridge.cpp")
        .compile("libcpp_bridge.a");
        
    let PROJECT_DIR = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let ethSTARK_LIB_DIR = PROJECT_DIR + "/ethSTARK/";        
    println!("cargo:rustc-link-lib=static=cpp_bridge"); 
    println!("cargo:rustc-link-lib=dylib=stdc++");    
    println!("cargo:rustc-link-search=native={}", ethSTARK_LIB_DIR);
    // --- ethS libs
    println!("cargo:rustc-link-lib=dylib=air_test_utils"); 
    println!("cargo:rustc-link-lib=dylib=domains"); 
    println!("cargo:rustc-link-lib=dylib=input_utils"); 
    println!("cargo:rustc-link-lib=dylib=starkware_gbenchmark"); 
    println!("cargo:rustc-link-lib=dylib=starkware_gtest"); 
    // rescue_verifier and its dependencies   
    println!("cargo:rustc-link-lib=dylib=rescue_verifier");       // w2
    println!("cargo:rustc-link-lib=dylib=rescue_prover");         // w2
    println!("cargo:rustc-link-lib=dylib=prover_main_helper"); 
    println!("cargo:rustc-link-lib=dylib=verifier_main_helper");       
    println!("cargo:rustc-link-lib=dylib=stark"); 
    println!("cargo:rustc-link-lib=dylib=fri"); 
    println!("cargo:rustc-link-lib=dylib=oods"); 
    println!("cargo:rustc-link-lib=dylib=composition_oracle"); 
    println!("cargo:rustc-link-lib=dylib=table"); 
    println!("cargo:rustc-link-lib=dylib=packaging_commitment_scheme"); 
    println!("cargo:rustc-link-lib=dylib=merkle_commitment_scheme"); 
    println!("cargo:rustc-link-lib=dylib=merkle_tree"); 
    println!("cargo:rustc-link-lib=dylib=packer_hasher"); 
    println!("cargo:rustc-link-lib=dylib=breaker"); 
    println!("cargo:rustc-link-lib=dylib=channel"); 
    println!("cargo:rustc-link-lib=dylib=proof_of_work"); 
    println!("cargo:rustc-link-lib=dylib=profiling"); 
    println!("cargo:rustc-link-lib=dylib=proof_system"); 
    println!("cargo:rustc-link-lib=dylib=flag_validators");     
    println!("cargo:rustc-link-lib=dylib=task_manager"); 
    println!("cargo:rustc-link-lib=dylib=error_handling"); 
    println!("cargo:rustc-link-lib=dylib=dw"); 
    println!("cargo:rustc-link-lib=dylib=rescue_statement"); 
    println!("cargo:rustc-link-lib=dylib=rescue_air"); 
    println!("cargo:rustc-link-lib=dylib=periodic_column"); 
    println!("cargo:rustc-link-lib=dylib=neighbors"); 
    println!("cargo:rustc-link-lib=dylib=domains"); 
    println!("cargo:rustc-link-lib=dylib=json"); 
    println!("cargo:rustc-link-lib=dylib=fields"); 
    println!("cargo:rustc-link-lib=dylib=to_from_string"); 
    println!("cargo:rustc-link-lib=dylib=prng"); 
    println!("cargo:rustc-link-lib=dylib=glog");
    println!("cargo:rustc-link-lib=dylib=gflags");
    println!("cargo:rustc-link-lib=dylib=jsoncpp"); 
    println!("cargo:rustc-link-lib=dylib=blake2s"); 
}
