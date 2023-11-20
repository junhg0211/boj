#include <iostream>
#include <string>

bool tick() {
    std::string string;
    std::cin >> string;
    int string_length = string.size();

    int depth = 0;
    for (int i = 0; i < string_length; i++) {
        if (string[i] == '(') {
            depth++;
        } else {
            depth--;
        }

        if (depth < 0) {
            return false;
        }
    }

    if (depth != 0) {
        return false;
    }

    return true;
}

int main() {
    int testcase_count;
    std::cin >> testcase_count;
    for (int i = 0; i < testcase_count; i++) {
        if (tick()) {
            std::cout << "YES\n";
        } else {
            std::cout << "NO\n";
        }
    }

    return 0;
}
