#include <string>

// ---------------------------- ethS ----------------------------//
// --- libhello.a (eth)

extern void hello();
extern "C" void hello_c() {
    hello();
}

// --- libtest_air.a (eth)

extern void feth(std::string const& str, int reps);
extern "C" void feth_c(const char* name, int reps) {
    feth(name, reps);
}
// ---------------------------- ethS end ----------------------------//

// --- libhello.so

extern void say_hello(std::string const& str, int reps);

namespace Foo {
    extern void say(std::string const& str, int reps);
}

extern "C" void say_hello_c(const char* name, int reps) {
    say_hello(name, reps);
    // Foo::say(name, reps);
}

// --- libtest.a

extern void f1(std::string const& str, int reps);

extern "C" void f1_c(const char* name, int reps) {
    f1(name, reps);
}

// --- libcmake-hello.a

extern void f3(std::string const& str, int reps);

extern "C" void f3_c(const char* name, int reps) {
    f3(name, reps);
}

// --- libhello2.a (cmake)

extern void hello2();
extern "C" void hello2_c() {
    hello2();
}

// // --- libtest2.a

// extern void f2(std::string const& str, int reps);

// extern "C" void f2_c(const char* name, int reps) {
//     f2(name, reps);
// }

