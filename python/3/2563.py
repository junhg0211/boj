from sys import stdin

input = stdin.readline


def main():
    count = int(input())
    filleds = set()

    for _ in range(count):
        x, y = map(int, input().split())

        for i in range(10):
            for j in range(10):
                filleds.add((x+i, y+j))

    print(len(filleds))


if __name__ == '__main__':
    main()
