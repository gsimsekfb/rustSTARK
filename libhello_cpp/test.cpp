#include <string>
#include <iostream>

extern void f2(std::string const& str, int reps);

void f1(std::string const& str, int reps) {
    for(int i = 0; i < reps; ++i)
        std::cout << "f1 " << str << "\n";

    f2(str, reps);
}