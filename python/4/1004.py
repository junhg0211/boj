from math import hypot


def is_in(px: int, py: int, pr: int, x: int, y: int) -> bool:
    dx, dy = x - px, y - py
    r = hypot(dx, dy)
    return r < pr


def main():
    for _ in range(int(input())):
        x1, y1, x2, y2 = map(int, input().split())
        count = 0
        for _ in range(int(input())):
            x, y, r = map(int, input().split())
            if is_in(x, y, r, x1, y1) != is_in(x, y, r, x2, y2):
                count += 1
        print(count)


if __name__ == '__main__':
    main()
