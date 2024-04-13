#include <stdio.h>
#include <stdlib.h>

int main() {
    int length;
    scanf("%d", &length);

    char *string = (char *)malloc(sizeof(char) * (length + 1));
    scanf("%s", string);

    int streak = 0;
    int good = 0;
    for (int i = 0; i < length; i++) {
        if (string[i] == 'o') {
            streak++;
        } else {
            streak = 0;
        }

        if (streak >= 3) {
            good = 1;
            break;
        }
    }

    if (good) {
        printf("Yes\n");
    } else {
        printf("No\n");
    }

    free(string);

    return 0;
}
