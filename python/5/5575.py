def tick():
    a, b, c, d, e, f = map(int, input().split())

    f -= c
    if f < 0:
        b += 1
        f += 60

    e -= b
    if e < 0:
        a += 1
        e += 60

    d -= a

    print(d, e, f)


def main():
    for _ in range(3):
        tick()


if __name__ == '__main__':
    main()
