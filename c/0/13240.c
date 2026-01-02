#include <stdio.h>

int main() {
    int width, height;
    scanf("%d %d", &height, &width);

    for (int i = 0; i < height; i++) {
        for (int j = 0; j < width; j++) {
            printf("%c", (i + j) % 2 == 0 ? '*' : '.');
        }
        printf("\n");
    }

    return 0;
}
