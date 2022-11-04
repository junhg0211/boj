#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

typedef struct _board {
    int x;
    int y;
    int size;
} Board;

typedef struct _boardqueue {
    Board *now;
    struct _boardqueue *next;
} BoardQueue;

BoardQueue* init() {
    BoardQueue *result = (BoardQueue *)malloc(sizeof(BoardQueue));
    result->next = NULL;
    return result;
}

BoardQueue* get_last(BoardQueue *queue) {
    while (queue->next != NULL) {
        queue = queue->next;
    }
    return queue;
}

void append(BoardQueue *queue, Board *board) {
    queue = get_last(queue);
    BoardQueue *new = init();
    new->now = board;
    queue->next = new;
}

Board* pop(BoardQueue *queue, BoardQueue **next_queue) {
    Board *result = queue->now;
    next_queue = &queue->next;
    free(queue);
    return result;
}

BoardQueue* divide(Board *board) {
    int ps = board->size / 3;
    BoardQueue *result = init();

    for (int i = 0; i < 3; i++) {
        for (int j = 0; j < 3; j++) {
            Board small_result;
            small_result.x = ps * i;
            small_result.y = ps * j;
            small_result.size = ps;
            append(result, &small_result);
        }
    }

    return result;
}

// bool is_all_same(int big_board[][], Board *board) {
    // for (int i = board->y; i < board->y + board->size; i++) {
        // for (int j = board->x; j < board->x + board->size; j++) {
            // if (big_board[board->y][board->x] != big_board[i][j]) {
                // return false;
            // }
        // }
    // }
    // return true;
// }

int main(void) {
    int size;
    scanf("%d", &size);

    int board[size][size];

    for (int i = 0; i < size; i++) {
        for (int j = 0; j < size; j++) {
            scanf("%d", &board[i][j]);
        }
    }

    BoardQueue *queue = init();
    queue->now->x = 0;
    queue->now->y = 0;
    queue->now->size = size;

    int counts[3] = {0};

    while (1) {
        Board *this_board = pop(queue, &queue->next);
        bool same = true;
        for (int i = this_board->y; i < this_board->y + this_board->size; i++) {
            for (int j = this_board->x; j < this_board->x + this_board->size; j++) {
                if (board[this_board->y][this_board->x] != board[i][j]) {
                    same = false;
                    break;
                }
            }
        }

        if (same) {
            int kind = board[this_board->y][this_board->x];
            if (kind == -1) kind = 2;
            counts[kind]++;
        }
    }

    for (int i = -1; i <= 1; i++) {
        printf("%d\n", counts[i%3]);
    }

    return 0;
}
