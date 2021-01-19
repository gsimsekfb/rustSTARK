#include <string>

// ---------------------------- ethS ---------------------------- //

// --- librescue_verifier.a

extern int rescue_verify(int argc, char** argv);
extern "C" int rescue_verify_c(int argc, char** argv) {
    return rescue_verify(argc, argv);
}

// ---------------------------- ethS end ----------------------- //

// --- libtest.a

extern void f1(std::string const& str, int reps);
extern "C" void f1_c(const char* name, int reps) {
    f1(name, reps);
}