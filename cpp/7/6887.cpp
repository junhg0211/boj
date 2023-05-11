#include <iostream>
#include <cmath>

int main()
{
    int number;
    std::cin >> number;

    int side_length = (int)std::sqrt(number);

    std::cout << "The largest square has side length " << side_length << ".\n";

    return 0;
}
