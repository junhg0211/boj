from math import inf


def main():
    while True:
        b, n = map(int, input().split())

        if b == 0 and n == 0:
            break

        a = 0
        previous = inf
        while True:
            value = abs(a**n - b)

            if value > previous:
                print(a - 1)
                break

            a += 1
            previous = value


if __name__ == "__main__":
    main()
