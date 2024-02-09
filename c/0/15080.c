#include <stdio.h>

int main() {
    int a, b, c;
    int x, y, z;

    scanf("%d : %d : %d", &a, &b, &c);
    scanf("%d : %d : %d", &x, &y, &z);

    int seconds = (x-a) * 3600 + (y-b) * 60 + (z-c);

    if (seconds < 0) {
        seconds += 3600 * 24;
    }

    printf("%d\n", seconds);

    return 0;
}
