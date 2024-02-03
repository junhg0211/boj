#include <iostream>
#include <string>

int main() {
    bool been[26] = {};

    std::string message;
    std::cin >> message;

    int length = message.length();
    for (int i = 0; i < length; i++) {
        been[message.at(i) - 'A'] = true;
    }

    for (int i = 0; i < 26; i++) {
        if (!been[i]) {
            std::cout << (char)('A' + i);
            break;
        }
    }

    return 0;
}
