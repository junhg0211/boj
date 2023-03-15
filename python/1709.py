from math import hypot

center = 0


def is_in(i, j):
    return hypot(i+1 - center, j+1 - center) < center < hypot(i - center, j - center)


def main():
    global center

    n = int(input())
    center = n//2

    count = sum(len(list(filter(lambda j: is_in(i, j), range(center-i)))) for i in range(center))

    print(count * 4)


if __name__ == '__main__':
    main()
