#include <iostream>

int gcd(int a, int b) {
    if (a < b) {
        int tmp = a;
        a = b;
        b = tmp;
    }

    while (b > 0) {
        int tmp = a;
        a = b;
        b = tmp%b;
    }

    return a;
}

void tick() {
    // get m, n, x, y
    int m, n, x, y;
    std::cin >> m >> n >> x >> y;

    // check validity
    int g = gcd(m, n);
    if (x > y && (x-y) % g != 0 || x < y && (y-x) % g != 0) {
        std::cout << "-1\n";
        return;
    }

    // calculate year
    while (x != y) {
        if (x < y) {
            x += m;
        } else {
            y += n;
        }
    }

    // print result
    std::cout << x << std::endl;
}

int main() {
    // get testcase count
    int testcase_count;
    std::cin >> testcase_count;

    // repeat `count` times
    for (int i = 0; i < testcase_count; i++) {
        tick();
    }

    return 0;
}
