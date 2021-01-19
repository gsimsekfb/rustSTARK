// ---------- rescue_prove
extern "C" {
    pub fn rescue_prove_c(
        argc: ::std::os::raw::c_int,
        argv: *const *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}

pub fn rescue_prove_rust() {
    use ::std::os::raw::c_char;
    use ::std::os::raw::c_int;
    use std::ffi::CString;
    let args = std::env::args().map(|arg| CString::new(arg).unwrap())
        .collect::<Vec<CString>>();
    let c_args = args.iter().map(|arg| arg.as_ptr())
        .collect::<Vec<*const c_char>>();
    unsafe { 
        println!("----------- unsafe C++ block");
        rescue_prove_c(c_args.len() as c_int, c_args.as_ptr()); 
        println!("----------- end of unsafe C++");
    }    
}

// ---------- rescue_verify
extern "C" {
    pub fn rescue_verify_c(
        argc: ::std::os::raw::c_int,
        argv: *const *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}

pub fn rescue_verify_rust() {
    use ::std::os::raw::c_char;
    use ::std::os::raw::c_int;
    use std::ffi::CString;
    let args = std::env::args().map(|arg| CString::new(arg).unwrap())
        .collect::<Vec<CString>>();
    let c_args = args.iter().map(|arg| arg.as_ptr())
        .collect::<Vec<*const c_char>>();
    unsafe { 
        println!("\n----------- unsafe C++ block  -----------");
        rescue_verify_c(c_args.len() as c_int, c_args.as_ptr()); 
        println!("----------- end of unsafe C++ -----------\n");
    }    
}

// --- main

fn main() {
    // rescue_prove_rust();
    rescue_verify_rust();
    
    println!(">>>>>>>>>>>>>>>>>>>>>>>>>>");
    println!("End of rust main()\n");
}