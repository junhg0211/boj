#include <iostream>

int min(int a, int b) {
    return a > b ? b : a;
}

int main() {
    int a1, a2, a3;
    std::cin >> a1 >> a2 >> a3;

    int result = 2147483647;

    result = min(result, a2*2 + a3*4);
    result = min(result, a1*2 + a3*2);
    result = min(result, a1*4 + a2*2);

    std::cout << result << '\n';

    return 0;
}
