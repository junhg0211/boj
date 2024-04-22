#include <stdio.h>

int main() {
    int count;
    scanf("%d", &count);

    int a = 0;
    int b = 0;
    for (int i = 0; i < count; i++) {
        int c1, c2;
        scanf("%d %d", &c1, &c2);

        if (c1 > c2) {
            a += c1 + c2;
        } else if (c1 < c2) {
            b += c1 + c2;
        } else {
            a += c1;
            b += c2;
        }
    }

    printf("%d %d\n", a, b);

    return 0;
}
