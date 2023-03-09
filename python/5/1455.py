from sys import stdin

input = stdin.readline


def main():
    height, width = map(int, input().split())
    board = [list(map(lambda x: bool(int(x)), input().rstrip())) for _ in range(height)]

    flips = [0 for _ in range(width)]

    for j, row in enumerate(reversed(board)):
        j = height-j-1
        flips_until_now = 0
        for i in range(width-1, -1, -1):
            flips_until_now += flips[i]

            if row[i] ^ bool(flips_until_now % 2):
                flips[i] += 1
                flips_until_now += 1

    print(sum(flips))


if __name__ == '__main__':
    main()
