#include <string>
#include <iostream>

void f2(std::string const& str, int reps) {
    for(int i = 0; i < reps; ++i)
        std::cout << "f2 " << str << "\n";
}