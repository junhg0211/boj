#include <stdio.h>

int main() {
    int a[8];

    for (int i = 0; i < 8; i++) {
        scanf("%d", &a[i]);
    }

    int ascending = 1;
    int descending = 1;
    for (int j = 0; j < 7; j++) {
        if (a[j] < a[j + 1]) {
            descending = 0;
        } else if (a[j] > a[j + 1]) {
            ascending = 0;
        }
    }

    if (ascending) {
        printf("ascending\n");
    } else if (descending) {
        printf("descending\n");
    } else {
        printf("mixed\n");
    }

    return 0;
}
