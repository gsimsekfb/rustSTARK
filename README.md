## Call C++ from Rust using shared library

### Steps  

```
Ubuntu 20.04

// Create C++ shared library for libhello_cpp/hello.cpp 
cd libhello_cpp
g++ -shared -fPIC -o libhello.so hello.cpp 

// Run rust app
cargo r
```

### Sources
https://crates.io/crates/cc  
https://www.youtube.com/watch?v=iCZkqDMJiF8  

