#include <stdio.h>

int main() {
    int number;
    scanf("%d", &number);

    int k;
    int result = 0;
    for (int i = 1; i < number; i++) {
        for (int j = 1; j < i+1; j++) {
            k = number - i - j;

            if (!(k <= j))
                continue;
            if (!(i < j+k))
                continue;

            result++;
        }
    }

    printf("%d\n", result);

    return 0;
}
