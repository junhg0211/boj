#include <iostream>
#include <iomanip>

int main()
{
    // getting numbers
    int number;
    int divisor;
    std::cin >> number >> divisor;

    number /= 100;

    // find divisable
    int i;
    for (i = 0; i < 100; i++)
    {
        if ((number * 100 + i) % divisor == 0)
            break;
    }

    std::cout << std::setw(2) << std::setfill('0') << i << std::endl;

    return 0;
}
