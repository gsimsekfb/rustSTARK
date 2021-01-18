#include <string>
#include <iostream>

extern void hello();    // eth: libhello.a
extern void hello2();
extern void hellota();  // eth: libtest_air.a

// void f1() {
//     std::cout << "f1() \n";
// }

int main() {
    // hello();
    hellota();
}