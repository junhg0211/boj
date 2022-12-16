from sys import stdin

input = stdin.readline


def is_lunar(year):
    return year % 4 == 0


def is_valid(date):
    y, m, d = date

    if m > 12 or m == 0:
        return False

    return 0 < d <= (31, 28 + int(is_lunar(y)), 31, 30, 31, 30, 31, 31, 30, 31, 30, 31)[m-1]


def get_possibles(a, b, c):
    return filter(is_valid, ((a, b, c), (c, b, a), (c, a, b)))


def tick(y, m, d):
    today = y, m, d

    unsafe = False
    valid = False
    for possible in get_possibles(*map(int, input().split())):
        valid = True
        if possible < today:
            unsafe = True
            break

    if valid:
        print('un' * unsafe + 'safe')
    else:
        print('invalid')


def main():
    y, m, d = map(int, input().split())

    for _ in range(int(input())):
        tick(y, m, d)


if __name__ == '__main__':
    main()
