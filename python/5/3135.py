from sys import stdin

input = stdin.readline


def main():
    a, b = map(int, input().split())

    count = int(input())
    button_channels = [int(input()) for _ in range(count)]

    result = abs(a-b)

    for button_channel in button_channels:
        result = min(result, abs(button_channel-b)+1)

    print(result)


if __name__ == '__main__':
    main()
