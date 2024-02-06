from math import ceil, floor


def get_rate(x: int, y: int) -> int:
    return floor(100 * y / x)


def is_valid(x: int, y: int, n: int):
    return get_rate(x, y) < get_rate(x+n, y+n)


def main():
    x, y = map(int, input().split())
    if get_rate(x, y) == 100:
        print('-1')
        return

    try:
        yes = ceil(x/100 / (99/100 - y/x))
    except ZeroDivisionError:
        print('-1')
        return

    if yes <= 0:
        print('-1')
        return

    no = 0

    while yes - no > 1:
        n = (yes + no) // 2
        if is_valid(x, y, n):
            yes = n
        else:
            no = n

    print(yes)


if __name__ == '__main__':
    main()
