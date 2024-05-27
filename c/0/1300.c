#include <stdio.h>

int min(int a, int b) { return a < b ? a : b; }

int main() {
    int n, k;
    scanf("%d %d", &n, &k);

    int start = 1;
    int end = k;
    while (start != end) {
        int mid = (start + end) / 2;

        long long count = 0;
        for (int i = 1; i <= n; i++) {
            count += min(mid / i, n);
        }

        if (k <= count) {
            end = mid;
        } else {
            start = mid + 1;
        }
    }

    printf("%d\n", start);

    return 0;
}
