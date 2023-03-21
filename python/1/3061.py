from sys import stdin

input = stdin.readline


def tick():
    count = int(input())
    numbers = list(map(int, input().split()))

    moves = 0

    for i in range(1, count+1):
        index = numbers.index(i)
        numbers.pop(index)

        moves += abs(index - (i-1))

        numbers.insert(i-1, i)

    print(moves)


def main():
    for _ in range(int(input())):
        tick()


if __name__ == '__main__':
    main()
