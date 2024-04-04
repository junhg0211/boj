#include <stdio.h>

int main() {
    double number;
    double previous = -100;

    while (1) {
        scanf("%lf", &number);

        if (number == 999) {
            break;
        }

        if (previous != -100) {
            printf("%.2lf\n", number - previous);
        }

        previous = number;
    }

    return 0;
}
