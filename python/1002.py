def tick():
    x1, y1, r1, x2, y2, r2 = map(int, input().split())

    dq = (x1-x2)**2 + (y1-y2)**2

    if x1 == x2 and y1 == y2 and r1 == r2:
        print(-1)
    elif abs(r1-r2)**2 > dq or (r1+r2)**2 < dq:
        print(0)
    elif dq == abs(r1-r2)**2 or dq == (r1+r2)**2:
        print(1)
    else:
        print(2)


def main():
    for _ in range(int(input())):
        tick()


if __name__ == '__main__':
    main()
