from math import hypot


def main():
    w, h, x, y, p = map(int, input().split())
    radius = h / 2

    count = 0
    for _ in range(p):
        px, py = map(int, input().split())

        if x <= px <= x + w and y <= py <= y + h:
            count += 1
            continue
        if px <= x and hypot(x - px, y+radius - py) <= radius:
            count += 1
            continue
        if px >= x+w and hypot(x+w - px, y+radius - py) <= radius:
            count += 1
            continue

    print(count)


if __name__ == '__main__':
    main()
