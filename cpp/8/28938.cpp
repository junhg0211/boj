#include <iostream>

int main() {
    int count;
    std::cin >> count;

    int sum = 0;
    for (int i = 0; i < count; i++) {
        int step;
        std::cin >> step;
        sum += step;
    }

    if (sum < 0) {
        std::cout << "Left\n";
    } else if (sum > 0) {
        std::cout << "Right\n";
    } else {
        std::cout << "Stay\n";
    }

    return 0;
}
