## Call C++ from Rust using shared library method

In this app, [src/main.rs](https://github.com/simsekgokhan/rust_call_cpp_so/blob/main/src/main.rs) calls C++ function `void say_hello(std::string const& str, int reps)` from [libhello_cpp/hello.cpp](https://github.com/simsekgokhan/rust_call_cpp_so/blob/main/libhello_cpp/hello.cpp).
  
  
### How to use  

```
Ubuntu 20.04

// Edit libhello_cpp/hello.cpp as you like

// Create C++ shared library for libhello_cpp/hello.cpp 
cd libhello_cpp
g++ -shared -fPIC -o libhello.so hello.cpp 

// Add link path - edit the username
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/home/gsimsek/rust_call_cpp/libhello_cpp/

// Run rust app
cargo r

// Output
Hello gman
Hello gman
Hello gman
  // main.rs calling C++ function -
  // void say_hello(std::string const& str, int reps) -
  // from libhello_cpp/hello.cpp 



```

### Sources
https://crates.io/crates/cc  
https://www.youtube.com/watch?v=iCZkqDMJiF8  

