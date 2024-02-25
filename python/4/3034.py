from math import hypot


def main():
    count, width, height = map(int, input().split())

    diagonal = hypot(width, height)
    for _ in range(count):
        print('DA' if int(input()) <= diagonal else 'NE')


if __name__ == '__main__':
    main()
