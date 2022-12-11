from sys import stdin

input = stdin.readline


TERMINALS = (
    # I
    ((1, 1, 1, 1),),
    ((1,), (1,), (1,), (1,)),
    # O
    ((1, 1), (1, 1)),
    # L
    ((1, 1, 1), (1, 0, 0)),
    ((1, 0), (1, 0), (1, 1)),
    ((0, 0, 1), (1, 1, 1)),
    ((1, 1), (0, 1), (0, 1)),
    # J
    ((1, 0, 0), (1, 1, 1)),
    ((1, 1), (1, 0), (1, 0)),
    ((1, 1, 1), (0, 0, 1)),
    ((0, 1), (0, 1), (1, 1)),
    # S
    ((1, 0), (1, 1), (0, 1)),
    ((0, 1, 1), (1, 1, 0)),
    # Z
    ((0, 1), (1, 1), (1, 0)),
    ((1, 1, 0), (0, 1, 1)),
    # T
    ((1, 1, 1), (0, 1, 0)),
    ((1, 0), (1, 1), (1, 0)),
    ((0, 1, 0), (1, 1, 1)),
    ((0, 1), (1, 1), (0, 1))
)


def get_max(board, bw, bh, terminal):
    max_ = 0
    th = len(terminal)
    tw = len(terminal[0])
    for x in range(bw - tw + 1):
        for y in range(bh - th + 1):
            value = 0
            for i in range(th):
                for j in range(tw):
                    value += terminal[i][j] * board[y+i][x+j]
            max_ = max(value, max_)
    return max_



def main():
    height, width = map(int, input().split())
    board = [list(map(int, input().split())) for _ in range(height)]

    max_ = 0
    for terminal in TERMINALS:
        max_ = max(max_, get_max(board, width, height, terminal))

    print(max_)


if __name__ == '__main__':
    main()
