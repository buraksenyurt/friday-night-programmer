#include <iostream>

/*
# Windows 11

# Program kodunu derlemek için
g++ -o overloading .\overloading.cpp

# ve oluşan exe içerisindeki sembolleri *(symbols)* görmek için
nm overloading.exe

# Kodu denemek için
.\overloading.exe
*/

float add(float a, float b) {
    return a + b;
}

int add(int a, int b) {
    return a + b;
}

int main() {
    int result_1 = add(1, 2);
    float result_2 = add(3.14f, 3.14f);

    std::cout << "Total of 1 and 2: " << result_1 << std::endl;
    std::cout << "Total of 3.14 and 3.14: " << result_2 << std::endl;

    return 0;
}