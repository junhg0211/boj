#include <stdio.h>

int main() {
    long p1, q1, p2, q2;
    scanf("%ld %ld %ld %ld", &p1, &q1, &p2, &q2);

    long numerator = p1 * p2;
    long denominator = q1 * q2 * 2;

    printf("%d\n", numerator % denominator == 0);

    return 0;
}
