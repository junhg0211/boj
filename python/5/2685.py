from sys import stdin

input = stdin.readline


def nimsum(b, x, y):
    results = list()
    while x or y:
        x, i = divmod(x, b)
        y, j = divmod(y, b)
        results.append((i+j) % b)

    result = 0
    while results:
        result = result * b + results.pop()

    return result


def main():
    for _ in range(int(input())):
        print(nimsum(*map(int, input().split())))


if __name__ == '__main__':
    main()
