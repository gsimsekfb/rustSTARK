#include <string>

extern void say_hello(std::string const& str, int reps);

extern "C" void say_hello_c(const char* name, int reps) {
    say_hello(name, reps);
}