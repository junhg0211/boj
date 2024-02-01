#include <iostream>

void tick() {
    int area, area_price;
    std::cin >> area >> area_price;

    double ppd_area = (double)area_price / area;

    int radius, radius_price;
    std::cin >> radius >> radius_price;

    double ppd_radius = (double)radius_price / (radius * radius * 3.1415926535);

    if (ppd_area > ppd_radius) {
        std::cout << "Whole pizza\n";
    } else {
        std::cout << "Slice of pizza\n";
    }
}

int main() {
    int testcase_count;
    std::cin >> testcase_count;

    for (int i = 0; i < testcase_count; i++) {
        tick();
    }

    return 0;
}
