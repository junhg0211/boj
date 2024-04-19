#include <iostream>
#include <stack>
#include <string>

int main() {
    int count;
    std::cin >> count;

    std::stack<int> stack;

    for (int i = 0; i < count; i++) {
        while (!stack.empty()) {
            stack.pop();
        }

        std::string string;
        std::cin >> string;

        bool good = true;

        for (int j = 0; j < string.length(); j++) {
            char c = string.at(j);

            if (c == '(') {
                stack.push(c);
                continue;
            }

            else {
                if (stack.empty()) {
                    good = false;
                    break;
                }

                stack.pop();
            }
        }

        if (!stack.empty()) {
            good = false;
        }

        if (good) {
            std::cout << "YES\n";
        } else {
            std::cout << "NO\n";
        }
    }

    return 0;
}
