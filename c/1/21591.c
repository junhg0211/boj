#include <stdio.h>

int main() {
    // -- get data
    int computer_width, computer_height;
    int sticker_width, sticker_height;

    scanf("%d %d %d %d", &computer_width, &computer_height, &sticker_width,
            &sticker_height);

    // -- calculate possibility
    int width_possible = computer_width - 2 >= sticker_width;
    int height_possible = computer_height - 2 >= sticker_height;

    if (width_possible && height_possible) {
        printf("1\n");
    } else {
        printf("0\n");
    }

    return 0;
}
