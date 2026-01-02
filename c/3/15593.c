#include <stdio.h>
#include <limits.h>

int main() {
    int cow_count;
    scanf("%d", &cow_count);

    int covering[200];

    for (int i = 0; i < cow_count; i++) {
        int start, end;
        scanf("%d %d", &start, &end);

        covering[i * 2] = start;
        covering[i * 2 + 1] = end;
    }

    int max_covering = 0;
    for (int i = 0; i < cow_count; i++) {
        int works[1001] = {};
        for (int j = 0; j < cow_count; j++) {
            if (i == j) continue;

            for (int k = covering[j * 2]; k < covering[j * 2 + 1]; k++) {
                works[k] = 1;
            }
        }

        int now_covering = 0;
        for (int j = 0; j <= 1000; j++) {
            if (works[j]) {
                now_covering++;
            }
        }

        if (now_covering > max_covering) {
            max_covering = now_covering;
        }
    }

    printf("%d\n", max_covering);

    return 0;
}
