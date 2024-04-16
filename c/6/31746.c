#include <stdio.h>
#include <string.h>

int main() {
    int count;
    scanf("%d", &count);

    char message[] = "SciComLove";
    int length = strlen(message);

    if (count % 2 == 0) {
        printf("%s\n", message);
    } else {
        for (int i = length - 1; i >= 0; i--) {
            printf("%c", message[i]);
        }
        printf("\n");
    }

    return 0;
}
