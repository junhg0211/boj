#include <stdio.h>

int get_area(int** board, int x, int y, int width, int height) {
    board[y][x] = 0;
    int area = 1;
    if (x > 0 && board[y][x-1] == 1) area += get_area(board, x-1, y, width, height);
    if (y > 0 && board[y-1][x] == 1) area += get_area(board, x, y-1, width, height);
    if (x + 1 < width && board[y][x+1]) area += get_area(board, x+1, y, width, height);
    if (y + 1 < height && board[y+1][x]) area += get_area(board, x, y+1, width, height);
    return area;
}

int main() {
    int board[501][501];
    int width, height;
    scanf("%d %d", &width, &height);
    for (int i = 0; i < height; i++) {
        for (int j = 0; j < width; j++) {
            scanf("%d", &board[i][j]);
        }
    }

    int max_area = 0, count = 0;
    for (int i = 0; i < height; i++) {
        for (int j = 0; j < width; j++) {
            if (board[i][j] == 1) {
                int area = get_area(board, i, j, width, height);
                if (area > max_area) max_area = area;
                count++;
            }
        }
    }
    printf("%d\n%d", count, max_area);
    return 0;
}
