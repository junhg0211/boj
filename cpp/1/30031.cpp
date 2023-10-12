#include <iostream>

int main() {
    int cash_count;
    std::cin >> cash_count;

    int total_money = 0;
    for (int i = 0; i < cash_count; i++) {
        int width, height;
        std::cin >> width >> height;

        if (width == 136) {
            total_money += 1000;
        } else if (width == 142) {
            total_money += 5000;
        } else if (width == 148) {
            total_money += 10000;
        } else {
            total_money += 50000;
        }
    }

    std::cout << total_money << "\n";

    return 0;
}
