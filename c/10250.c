#include <stdio.h>

int main() {
    int count;
    scanf("%d", &count);

    for (int i = 0; i < count; i++) {
        int h, w, n;
        scanf("%d %d %d", &h, &w, &n);

        int row = (n-1)%h + 1, column = (n-1)/h + 1;

        printf("%d%02d\n", row, column);
    }
    
    return 0;
}
