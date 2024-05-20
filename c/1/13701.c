#include <stdio.h>
#include <stdlib.h>

int main() {
    char *numbers = (char *)malloc(33554432 / 8);

    int number;
    while (scanf("%d", &number) != EOF) {
        if (!(numbers[number / 8] & (1 << (number % 8)))) {
            printf("%d ", number);
        }

        numbers[number / 8] |= 1 << (number % 8);
    }

    printf("\n");

    free(numbers);

    return 0;
}
