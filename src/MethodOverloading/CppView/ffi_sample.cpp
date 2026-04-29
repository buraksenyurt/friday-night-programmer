#include <iostream>
#include <windows.h>

typedef void (*PaymentWithAmountFunc)(double);
typedef void (*PaymentWithBonusFunc)(int);

int main()
{
    HINSTANCE dotNetLib = LoadLibrary(TEXT("FinanceLib.dll"));
    if (!dotNetLib)
    {
        std::cout << "DLL could not be loaded!" << std::endl;
        return 1;
    }

    PaymentWithAmountFunc pwAmount = (PaymentWithAmountFunc)GetProcAddress(dotNetLib, "ProcessPayment_WithAmount");
    PaymentWithBonusFunc pwBonus = (PaymentWithBonusFunc)GetProcAddress(dotNetLib, "ProcessPayment_WithBonus");

    if (pwAmount && pwBonus)
    {
        std::cout << "Calling C# functions...\n";

        pwAmount(99.99);
        pwBonus(10);
    }
    else
    {
        std::cout << "Functions not found in DLL." << std::endl;
    }

    FreeLibrary(dotNetLib);
    return 0;
}