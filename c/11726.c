#include <stdio.h>

int main(void) {
    int until;

    scanf("%d", &until);

    int a = 1, b = 1;

    for (int i = 0; i < until; i++) {
        int tmp = a + b % 10007;
        a = b;
        b = tmp;
    }

    printf("%d", a % 10007);

    return 0;
}
