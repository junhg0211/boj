#include <stdio.h>
#include <string.h>

int main() {
    char message[1000010];

    fgets(message, 1000010, stdin);
    int length = strlen(message);

    int start = 0;
    int result = 0;
    for (int i = 0; i < length; i++) {
        char c = message[i];

        if ((c == ' ' || c == '\n') && start) {
            result++;
            start = 0;
        }

        if ('a' <= c && c <= 'z' || 'A' <= c && c <= 'Z') {
            start = 1;
        }
    }

    printf("%d\n", result);

    return 0;
}
