#include <iostream>
#include <cstring>

void doSomething() {
    char buffer[8];
    std::strcpy(buffer, "Rust is perfect!");

    std::cout << "Buffer: " << buffer << std::endl;
}

int main() {
    doSomething();
    return 0;
}

