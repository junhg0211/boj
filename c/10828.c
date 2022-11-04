#include <stdio.h>

int diff(char* string1, char* string2, int length) {
    for (int i = 0; i < length; i++) {
        if (string1[i] != string2[i]) {
            return i;
        }
        if (string1[i] == '\0') {
            return -1;
        }
    }
    return -1;
}

int main() {
    // initialize socket
    int stack[10001];
    int stack_length = 0;

    // initialize commands count
    int count;
    scanf("%d\n", &count);

    // handle commands
    for (int i = 0; i < count; i++) {
        char command[10];
        scanf("%s", command);
        // scanf stops working at the space character

        if (diff(command, "push", 4) == -1) {
            scanf("%d", &stack[stack_length++]);
        } else if (diff(command, "pop", 3) == -1) {
            if (stack_length == 0) {
                printf("-1\n");
            } else {
                printf("%d\n", stack[--stack_length]);
            }
        } else if (diff(command, "size", 4) == -1) {
            printf("%d\n", stack_length);
        } else if (diff(command, "empty", 5) == -1) {
            printf(stack_length ? "0\n" : "1\n");
        } else if (diff(command, "top", 3)) {
            if (stack_length == 0) {
                printf("-1\n");
            } else {
                printf("%d\n", stack[stack_length - 1]);
            }
        }
    }

    return 0;
}
