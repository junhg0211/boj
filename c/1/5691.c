#include <stdio.h>

int main() {
    while (1) {
        int a, b;
        scanf("%d %d", &a, &b);

        if (a == 0 && b == 0) {
            break;
        }

        if (a < b) {
            int tmp = a;
            a = b;
            b = tmp;
        }

        printf("%d\n", b - (a - b));
    }

    return 0;
}
