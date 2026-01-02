#include <stdio.h>

int max(int a, int b) {
    return a < b ? b : a;
}

int max4(int a, int b, int c, int d) {
    return max(max(max(a, b), c), d);
}

void do_session() {
    int days;
    scanf("%d", &days);

    int benefit = 0;
    for (int i = 0; i < days; i++) {
        int a, b, c;
        scanf("%d %d %d", &a, &b, &c);

        benefit += max4(a, b, c, 0);
    }

    printf("%d\n", benefit);
}

int main() {
    int session_count;

    scanf("%d", &session_count);

    for (int i = 0; i < session_count; i++) {
        do_session();
    }

    return 0;
}
