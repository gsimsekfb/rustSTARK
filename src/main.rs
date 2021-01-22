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

pub fn rescue_verify() -> bool {
    unsafe { 
        println!("\n----------- unsafe C++ call   -----------\n");

        // proof
        let pp = constants::PROOF_HEX.to_owned() + "\0";
        let proof = pp.as_ptr() as *const _;
        // public_input
        let x = r#" {"chain_length":3,"output":["0xdb0a16d9f9cedae","0x244b64cb5a39a2b","0x1f6c22cd3cfdde49","0x4bf27b6fae084cb"]} "#;
        let xx = x.to_owned() + "\0";
        let public_input = xx.as_ptr() as *const _;
        // 
        // parameters
        let y = r#" {"stark":{"fri":{"fri_step_list":[1,2,2],"last_layer_degree_bound":1,"n_queries":30,"proof_of_work_bits":20},"log_n_cosets":2}} "#;
        let yy = y.to_owned() + "\0";
        let parameters = yy.as_ptr() as *const _;
        // annotation_file_name
        let a = "";     // "annotation.txt"
        let aa = a.to_owned() + "\0";
        let annotation_file_name = aa.as_ptr() as *const _;

        let result = rescue_verify_c(proof, public_input, parameters, annotation_file_name);

        println!("\n----------- end of unsafe C++ -----------\n");

        result
    }    
}

// --- main
fn main() {    
    // let first_arg = std::env::args().nth(1).unwrap();

    // if first_arg == "v" {
        println!("--- Rust: Proof verified: {:?}", rescue_verify());
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