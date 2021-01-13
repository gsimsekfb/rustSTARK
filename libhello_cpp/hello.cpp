#include <string>
#include <iostream>

void say_hello(std::string const& str, int reps) {
    for(int i = 0; i < reps; ++i)
        std::cout << "Hello " << str << "\n";
}