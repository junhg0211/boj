#include <stdio.h>

void swap(int *x, int *y) {
    int tmp = *x;
    *x = *y;
    *y = tmp;
}

int main() {
    int a, b, c, tmp;

    while (1) {
        scanf("%d %d %d", &a, &b, &c);
        
        if (a == 0 && b == 0 && c == 0) break;
        
        if (b > c) swap(&c, &b);
        if (a > b) swap(&b, &a);
        if (b > c) swap(&c, &b);

        printf(a*a + b*b == c*c ? "right\n" : "wrong\n");
    }

    return 0;
}
