use ::std::os::raw::c_char;
use ::std::os::raw::c_int;
use std::ffi::CString;

#[allow(dead_code)]
fn print_type_of<T>(_: &T) {
    println!("--- Type: {}", std::any::type_name::<T>())
}

extern "C" {
    pub fn rescue_prove_c(
        argc: ::std::os::raw::c_int,
        argv: *const *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}

extern "C" {
    pub fn rescue_verify_c(
        argc: ::std::os::raw::c_int,
        argv: *const *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}

pub fn rescue_prove(
    parameter_file: &str,
    prover_config_file: &str,
    public_input_file: &str,
    private_input_file: &str,
    out_file: &str
) {
    let args = [
        "unused_param", 
        "--parameter_file", parameter_file,
        "--prover_config_file", prover_config_file,
        "--public_input_file", public_input_file, 
        "--private_input_file", private_input_file, 
        "--out_file", out_file,
        "--logtostderr",       
    ];
    let args_vec: Vec<_> = args.iter().map(|&e| CString::new(e).unwrap())
                                      .collect();    
    let c_args = args_vec.iter().map(|arg| arg.as_ptr())
                                .collect::<Vec<*const c_char>>();
    unsafe { 
        println!("\n----------- unsafe C++ call   -----------\n");
        rescue_prove_c(c_args.len() as c_int, c_args.as_ptr()); 
        println!("\n----------- end of unsafe C++ -----------\n");
    }    
}

pub fn rescue_verify(in_file: &str) {
    let args = [
        "unused_param", 
        "--in_file", in_file,
        "--logtostderr",
    ];
    let args_vec: Vec<_> = args.iter().map(|&e| CString::new(e).unwrap())
                                      .collect();
    let c_args = args_vec.iter().map(|arg| arg.as_ptr())
                                .collect::<Vec<*const c_char>>();
    unsafe { 
        println!("\n----------- unsafe C++ call   -----------\n");
        rescue_verify_c(c_args.len() as c_int, c_args.as_ptr()); 
        println!("\n----------- end of unsafe C++ -----------\n");
    }    
}

// --- main
fn main() {    
    rescue_verify("/opt/example/proof.json");

    // rescue_prove(
    //     "/opt/example/rescue_params.json",
    //     "/opt/example/rescue_prover_config.json",      
    //     "/opt/example/rescue_public_input.json", 
    //     "/opt/example/rescue_private_input.json",
    //     "/opt/example/proof.json",        
    // );
    
    // ---- helpers
    // print_type_of(&c_args);        
    // println!("c_args: {:?}", c_args); 
    println!(">>>>>>>>>>>>>>>>>>>>>>>>>>");
    println!("End of rust main()\n");
}