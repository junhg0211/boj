#include <iostream>

int gcd(int a, int b)
{
    if (a < b)
        return gcd(b, a);

    int r;
    while (b != 0)
    {
        r = a % b;
        a = b;
        b = r;
    }

    return a;
}

int main()
{
    // get count of numbers
    int count;
    std::cin >> count;

    // calculate lcm
    int a, b;
    for (int i = 0; i < count; i++)
    {
        std::cin >> a >> b;
        std::cout << a * b / gcd(a, b) << std::endl;
    }

    return 0;
}
