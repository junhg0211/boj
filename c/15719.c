#include <stdio.h>
#include <stdlib.h>

int main() {
    int count;
    int number;
    scanf("%d", &count);

    char *numbers = (char *)malloc(sizeof(char) * count);

    for (int i = 0; i < count; i++) {
        scanf("%d", &number);

        if (numbers[number]) {
            printf("%d\n", number);
            break;
        }

        numbers[number] = 1;
    }

    free(numbers);

    return 0;
}
