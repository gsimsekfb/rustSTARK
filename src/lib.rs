pub mod example_proof;

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
    public_input_json: &str, 
    parameters_json: &str, 
    annotation_file_name: &str
) -> bool {
    // Convert &str -> CString -> *const c_char
    let args = [proof_hex, public_input_json, parameters_json, annotation_file_name];
    let args_vec = args.iter().map(|&arg| CString::new(arg).unwrap())
                              .collect::<Vec<CString>>();
    let c_args = args_vec.iter().map(|arg| arg.as_ptr())
                                .collect::<Vec<*const c_char>>();   
    let [proof_hex, public_input_json, parameters_json, annotation_file_name] 
        = [c_args[0], c_args[1], c_args[2], c_args[3]];
        
    unsafe { 
        println!("\n----------- unsafe C++ call   -----------\n");
        let result = rescue_verify_c(
            proof_hex, public_input_json, parameters_json, annotation_file_name
        );
        println!("\n----------- end of unsafe C++ -----------\n");

        result
    }    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify() {
        let result = rescue_verify(
            example_proof::PROOF_HEX, 
            example_proof::PUBLIC_INPUT,
            example_proof::PARAMETERS, 
            ""
        );        
        assert!(result);        
    }

    // Todo: enable later, test is working but too noisy 
    // #[test]
    // fn bad_verify_with_cpp_exception() {
    //     let result = rescue_verify(
    //         "foo", 
    //         example_proof::PUBLIC_INPUT,
    //         example_proof::PARAMETERS, 
    //         ""
    //     );        
    //     assert_eq!(result, false);
    // }

    #[test]
    fn bad_verify_wrong_proof() {
        let result = rescue_verify(
            "0xab",
            example_proof::PUBLIC_INPUT,
            example_proof::PARAMETERS, 
            ""
        );        
        assert_eq!(result, false);
    }    

    #[test]
    fn prove() {
        let _result = rescue_prove(
            "/opt/example/rescue_params.json",
            "/opt/example/rescue_prover_config.json",      
            "/opt/example/rescue_public_input.json", 
            "/opt/example/rescue_private_input.json",
            "/opt/example/proof.json",        
        );     
        // assert_eq!(result, 0);   // todo
    }    
    
}
