#include <stdio.h>

int main(void) {
    int until;

    scanf("%d", &until);

    int a = 1, b = 1;

    for (int i = 0; i < until; i++) {
        int tmp = 2*a + b;
        a = b % 10007;
        b = tmp;
    }

    printf("%d", a);

    return 0;
}

