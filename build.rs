/*
    https://crates.io/crates/cc
    A library to compile C/C++/assembly into a Rust library/application.
    > Documentation:
    A simple library meant to be used as a build dependency with Cargo 
    packages in order to build a set of C/C++ files into a static archive. 
    This crate calls out to the most relevant compiler for a platform, 
    for example using cl on MSVC.
*/

fn main() {
    cc::Build::new()
        .cpp(true)
        .file("hello.cpp")
        .compile("libhelloc.a");

    println!("cargo:rustc-link-lib=static=helloc"); 
    // println!("cargo:rustc-link-search=native=/home/gsimsek/cpplib/");
    println!("cargo:rustc-link-search=native=/home/gsimsek/rust_call_cpp/libhello_cpp/");
    println!("cargo:rustc-link-lib=dylib=hello"); 
    println!("cargo:rustc-link-lib=dylib=stdc++");  
}