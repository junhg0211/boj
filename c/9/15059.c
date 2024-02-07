#include <stdio.h>

int relu(int x) {
    if (x < 0) {
        return 0;
    }

    return x;
}

int main() {
    int a, b, c;
    scanf("%d %d %d", &a, &b, &c);

    int x, y, z;
    scanf("%d %d %d", &x, &y, &z);

    int result = relu(x - a) + relu(y - b) + relu(z - c);
    printf("%d\n", result);

    return 0;
}
