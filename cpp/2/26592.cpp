#include <iostream>

void tick() {
    double area, base;
    std::cin >> area >> base;

    double height = area * 2 / base;

    std::cout.precision(2);
    std::cout
        << "The height of the triangle is "
        << std::fixed
        << height
        << " units\n";
}

int main() {
    int testcase_count;
    std::cin >> testcase_count;

    for (int i = 0; i < testcase_count; i++) {
        tick();
    }

    return 0;
}
