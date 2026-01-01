#include <stdio.h>

int main() {
    int n, m, l;
    scanf("%d %d %d", &n, &m, &l);

    int gong_count[50] = {};
    int now_gong = 0;
    int throw_count = 0;

    while (1) {
        gong_count[now_gong]++;

        if (gong_count[now_gong] == m) {
            break;
        }

        if (gong_count[now_gong] % 2 == 1) {
            now_gong = (now_gong + l) % n;
        } else {
            now_gong = (now_gong - l + n) % n;
        }

        throw_count++;
    }

    printf("%d\n", throw_count);

    return 0;
}
