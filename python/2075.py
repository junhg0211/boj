from sys import stdin

input = stdin.readline


def main():
    n = int(input())

    numbers = [list(map(int, input().split())) for _ in range(n)]
    candidates = [n-1 for i in range(n)]

    for _ in range(4):
        index = max(range(n), key=lambda x: numbers[candidates[x]][x])
        candidates[index] -= 1

    index = max(range(n), key=lambda x: numbers[candidates[x]][x])
    print(numbers[candidates[index]][index])


if __name__ == '__main__':
    main()
