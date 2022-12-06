#include <stdio.h>

int factorial(int n) {
    int result = 1;
    for (int i = 2; i <= n; i++) {
        result *= i;
    }
    return result;
}

int main() {
    int n, k;

    scanf("%d %d", &n, &k);

    printf("%d", factorial(n) / (factorial(n-k) * factorial(k)));

    return 0;
}
