#include <stdio.h>

int main() {
    int cuts;
    scanf("%d", &cuts);

    int result = 0;
    for (int i = 0; i <= cuts; i++) {
        result += (i+2) / 2;
    }

    printf("%d\n", result);

    return 0;
}
