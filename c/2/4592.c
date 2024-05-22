#include <stdio.h>

int main() {
    while (1) {
        int count;
        scanf("%d", &count);

        if (count == 0) {
            break;
        }

        int previous = 0;
        for (int i = 0; i < count; i++) {
            int number;
            scanf("%d", &number);

            if (previous != number) {
                printf("%d ", number);
            }

            previous = number;
        }
        printf("$\n");
    }

    return 0;
}
