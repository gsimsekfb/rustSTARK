mod constants;

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
        proof:                  *const std::os::raw::c_char,
        public_input:           *const std::os::raw::c_char,
        parameters:             *const std::os::raw::c_char,
        annotation_file_name:   *const std::os::raw::c_char
    ) -> bool;
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

pub fn rescue_verify(
    proof_hex: &str,
    public_input: &str, 
    parameters: &str, 
    annotation_file_name: &str
) -> bool {
    // Convert &str -> CString -> *const c_char
    let args = [proof_hex, public_input, parameters, annotation_file_name];
    let args_vec = args.iter().map(|&arg| CString::new(arg).unwrap())
                              .collect::<Vec<CString>>();
    let c_args = args_vec.iter().map(|arg| arg.as_ptr())
                                .collect::<Vec<*const c_char>>();   
    let [proof_hex, public_input, parameters, annotation_file_name] 
        = [c_args[0], c_args[1], c_args[2], c_args[3]];
        
    unsafe { 
        println!("\n----------- unsafe C++ call   -----------\n");
        let result = rescue_verify_c(
            proof_hex, public_input, parameters, annotation_file_name
        );
        println!("\n----------- end of unsafe C++ -----------\n");

        result
    }    
}

// --- main
fn main() {    
    // let first_arg = std::env::args().nth(1).unwrap();

    // if first_arg == "v" {
        let result = rescue_verify(
            constants::PROOF_HEX, 
            constants::PUBLIC_INPUT,
            constants::PARAMETERS, 
            ""
        );
        println!("--- Rust: Proof verified: {:?}", result);
    // }
    // else {
    //     rescue_prove(
    //         "/opt/example/rescue_params.json",
    //         "/opt/example/rescue_prover_config.json",      
    //         "/opt/example/rescue_public_input.json", 
    //         "/opt/example/rescue_private_input.json",
    //         "/opt/example/proof.json",        
    //     );
    // }
    
    // ---- helpers
    // print_type_of(&args);        
    // println!("c_args: {:?}", c_args); 
    println!(">>>>>>>>>>>>>>>>>>>>>>>>>>");
    println!("End of rust main()\n");
}