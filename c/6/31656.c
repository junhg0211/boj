#include <stdio.h>
#include <string.h>

int main() {
    char string[1001];
    fgets(string, 1000, stdin);

    int length = strlen(string);
    char previous = 0;
    for (int i = 0; i < length; i++) {
        char c = string[i];

        if (c != previous) {
            printf("%c", c);
        }

        previous = c;
    }

    return 0;
}
