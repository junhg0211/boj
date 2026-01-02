#include <stdio.h>

int main() {
    int n;

    scanf("%d", &n);

    if (n >= 13) {
        printf("%d\n", n + 1);
    } else {
        printf("%d\n", n);
    }

    return 0;
}
