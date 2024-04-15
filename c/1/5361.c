#include <stdio.h>

void tick() {
    int a, b, c, d, e;
    scanf("%d %d %d %d %d", &a, &b, &c, &d, &e);

    double value =
        a * 350.34 + b * 230.90 + c * 190.55 + d * 125.30 + e * 180.90;

    printf("$%.2lf\n", value);
}

int main() {
    int testcase_count;
    scanf("%d", &testcase_count);

    for (int i = 0; i < testcase_count; i++) {
        tick();
    }

    return 0;
}
