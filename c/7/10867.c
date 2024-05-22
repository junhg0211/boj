#include <stdio.h>

int main() {
    int count;
    scanf("%d", &count);

    int beens[2001] = {};
    for (int i = 0; i < count; i++) {
        int number;
        scanf("%d", &number);

        beens[number + 1000] = 1;
    }

    for (int i = 0; i < 2001; i++) {
        if (beens[i]) {
            printf("%d ", i - 1000);
        }
    }

    printf("\n");

    return 0;
}
