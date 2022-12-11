#include <stdio.h>

int min(x, y) {
    return x < y ? x : y;
}

int main() {
    int x, y, w, h;

    scanf("%d %d %d %d", &x, &y, &w, &h);

    int result;
    result = min(min(min(x, w-x), y), h-y);

    printf("%d", result);

    return 0;
}
