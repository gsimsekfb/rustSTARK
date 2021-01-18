extern "C" {
    pub fn say_hello_c(name: *const ::std::os::raw::c_char, 
                       reps: ::std::os::raw::c_int);
}

extern "C" {
    pub fn f1_c(name: *const ::std::os::raw::c_char, 
                       reps: ::std::os::raw::c_int);
}

extern "C" {
    pub fn f3_c(name: *const ::std::os::raw::c_char, 
                       reps: ::std::os::raw::c_int);
}

extern "C" {
    pub fn feth_c(name: *const ::std::os::raw::c_char, 
                       reps: ::std::os::raw::c_int);
}

extern "C" { pub fn hello_c(); }     // ethS: libhello.a
extern "C" { pub fn hellota_c(); }   // ethS: libtest-air.a
extern "C" { pub fn hello2_c(); }    // cmake: libhello2.a

fn main() {
    unsafe {
        hello_c();
        // hellota_c();
        // hello2_c();
        // say_hello_c("gmann\0".as_ptr() as *const _, 3);
        // f1_c("gmann\0".as_ptr() as *const _, 1);
        feth_c("gmann\0".as_ptr() as *const _, 1);
        // f3_c("f3: cmake\0".as_ptr() as *const _, 1);
    }

    // println!("Hello, world!");
}