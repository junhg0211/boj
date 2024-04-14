#include <stdio.h>

int main() {
    int count;
    scanf("%d", &count);

    int sum = (count - 1) * 8;
    for (int i = 0; i < count; i++) {
        int hours;
        scanf("%d", &hours);
        sum += hours;
    }

    printf("%d %d\n", sum / 24, sum % 24);

    return 0;
}
