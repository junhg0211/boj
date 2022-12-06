from sys import stdin

input = stdin.readline


def main():
    height, width = map(int, input().split())
    board = [list(input()) for _ in range(height)]

    size = min(width, height) - 1
    while True:
        for x in range(width - size):
            for y in range(height - size):
                if board[y][x] == board[y+size][x] == board[y][x+size] == board[y+size][x+size]:
                    print((size + 1)**2)
                    return
        size -= 1


if __name__ == '__main__':
    main()
