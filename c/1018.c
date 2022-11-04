#include <stdio.h>

int main() {
    // size initialize
    int height, width;
    scanf("%d %d", &height, &width);

    // board initialize
    char board[height][width];
    char temp;

    scanf("%c", &temp);
    for (int i = 0; i < height; i++) {
        for (int j = 0; j < width; j++) {
            scanf("%c", &board[i][j]);
        }
        scanf("%c", &temp);
    }

    // check
    int min = width * height;
    for (int y = 0; y <= height - 8; y++) {
        for (int x = 0; x <= width - 8; x++) {
            int changes = 0;
            for (int i = 0; i < 8; i++) {
                for (int j = 0; j < 8; j++) {
                    // case i: top-left white
                    if ((i+j) % 2 == 0 && board[y+i][x+j] == 'B'
                            || (i+j) % 2 == 1 && board[y+i][x+j] == 'W') {
                        changes++;
                    }
                }
            }
            if (changes < min) min = changes;
            if (64 - changes < min) min = 64 - changes;
        }
    }

    printf("%d", min);

    return 0;
}
