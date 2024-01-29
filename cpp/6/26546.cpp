#include <iostream>
#include <string>

void tick() {
    std::string string;
    std::cin >> string;

    int start, end;
    std::cin >> start >> end;

    int length = string.length();
    for (int i = 0; i < length; i++) {
        if (i >= start && i < end) {
            continue;
        }

        std::cout << string.at(i);
    }
    std::cout << '\n';
}

int main() {
    int testcase_count;
    std::cin >> testcase_count;

    for (int i = 0; i < testcase_count; i++) {
        tick();
    }
}
