#include <iostream>
#include "library.h"


extern "C"
{
void hello() {
    std::cout << "Hello from C++!\n";
}
int test(int a, int b) {
    return a + b;
}
}