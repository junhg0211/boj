def main():
    t1, m1, t2, m2 = map(int, input().split())

    dt = t2 - t1
    dm = m2 - m1

    while dm < 0:
        dt -= 1
        dm += 60
    dt %= 24

    minutes = dt * 60 + dm

    print(minutes, minutes // 30)


if __name__ == '__main__':
    main()
