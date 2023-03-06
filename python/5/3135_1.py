from sys import stdin

input = stdin.readline


def main():
    a, b = map(int, input().split())

    result = abs(a-b)

    for button_channel in (int(input()) for _ in range(int(input()))):
        result = min(result, abs(button_channel-b)+1)

    print(result)


if __name__ == '__main__':
    main()
