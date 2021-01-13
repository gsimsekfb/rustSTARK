
extern "C" {
    // void say_hello_c(const char* name, int reps);
    pub fn say_hello_c(name: *const std::os::raw::c_char, reps: i32);
}

fn main() {
    unsafe {
        say_hello_c("gman\0".as_ptr() as *const _, 3);
    }

    // println!("Hello, world!");
}