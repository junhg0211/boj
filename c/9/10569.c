#include <stdio.h>

void tick() {
    int v, e;
    scanf("%d %d", &v, &e);

    printf("%d\n", 2 + e - v);
}

int main() {
    int count;
    scanf("%d", &count);

    for (int i = 0; i < count; i++) {
        tick();
    }

    return 0;
}
