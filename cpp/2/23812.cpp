#include <iostream>

void print_odd(int size) {
    for (int i = 0; i < size; i++) {
        for (int j = 0; j < size; j++) {
            std::cout << "@";
        }

        for (int j = 0; j < size; j++) {
            std::cout << "   ";
        }

        for (int j = 0; j < size; j++) {
            std::cout << "@";
        }

        std::cout << "\n";
    }
}

void print_even(int size) {
    for (int i = 0; i < size; i++) {
        for (int j = 0; j < size; j++) {
            std::cout << "@@@@@";
        }

        std::cout << "\n";
    }
}

int main() {
    int size;
    std::cin >> size;

    print_odd(size);
    print_even(size);
    print_odd(size);
    print_even(size);
    print_odd(size);

    return 0;
}
