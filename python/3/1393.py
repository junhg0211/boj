from math import hypot


def main():
    xs, ys = map(int, input().split())
    xe, ye, vx, vy = map(int, input().split())

    if hypot(xe+vx - xs, ye+vy - ys) > hypot(xe - xs, ye - ys):
        print(xe, ye)
        return

    if vx == 0:
        print(xe, ys)
        return

    if vy == 0:
        print(xs, ye)
        return

    tv = vy / vx
    tvt = -vx / vy

    x = (xs*tvt - ys - tv*xe + ye) / (tvt - tv)
    y = (ys*tv - tvt*tv*xs - ye*tvt + xe*tvt*tv) / (tv - tvt)

    print(round(x), round(y))


if __name__ == '__main__':
    main()
