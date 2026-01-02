#include <stdio.h>

int gcd(int a, int b) {
    if (a < b) {
        return gcd(b, a);
    }

    while (b != 0) {
        int temp = b;
        b = a % b;
        a = temp;
    }
    return a;
}

int main() {
    int n;
    scanf("%d", &n);

    int number;
    for (int i = 0; i < n; i++) {
        if (i == 0) {
            scanf("%d", &number);
            continue;
        }

        int next_number;
        scanf("%d", &next_number);
        number = gcd(number, next_number);
    }

    int stack[10000];
    int count = 0;

    for (int i = 1; i * i <= number; i++) {
        if (number % i == 0) {
            printf("%d\n", i);
            stack[count++] = number / i;
        }
    }

    for (int i = stack[count-1] * stack[count-1] == number ? 1 : 0; i < count; i++) {
        printf("%d\n", stack[count - i - 1]);
    }

    return 0;
}
