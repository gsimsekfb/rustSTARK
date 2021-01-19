extern "C" {
    pub fn rescue_verify_c(
        argc: ::std::os::raw::c_int,
        argv: *const *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}

// extern crate libc;
use ::std::os::raw::c_char;
use ::std::os::raw::c_int;
use std::ffi::CString;

fn main() {
    // create a vector of zero terminated strings
    let args = std::env::args().map(|arg| CString::new(arg).unwrap() ).collect::<Vec<CString>>();
    // convert the strings to raw pointers
    let c_args = args.iter().map(|arg| arg.as_ptr()).collect::<Vec<*const c_char>>();    

    let args2: Vec<String> = std::env::args().collect();
    println!("--- args2: {:?}", args2);

    unsafe {
        println!("------------------ unsafe C++ block");
        rescue_verify_c(c_args.len() as c_int, c_args.as_ptr());
        // feth_c("gmann\0".as_ptr() as *const _, 1);
        // hello_c();
        // hellota_c();
        // hello2_c();
        // say_hello_c("gmann\0".as_ptr() as *const _, 3);
        // f1_c("gmann\0".as_ptr() as *const _, 1);
        // f3_c("f3: cmake\0".as_ptr() as *const _, 1);
        println!("------------------ end of unsafe");
    }
    
    println!("----------------------------------------");
    println!("End of rust main\n");
}