#include <stdio.h>
#include <string.h>

int main() {
    int count;
    scanf("%d\n", &count);

    for (int i = 0; i < count; i++) {
        char message[1001];
        fgets(message, 1000, stdin);
        int length = strlen(message);
        message[--length] = 0;

        if (message[length / 2] == message[length / 2 - 1]) {
            printf("Do-it\n");
        } else {
            printf("Do-it-Not\n");
        }
    }

    return 0;
}
