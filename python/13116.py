from math import log2


def main():
    for _ in range(int(input())):
        a, b = map(int, input().split())

        if a < b:
            a, b = b, a

        a >>= (int(log2(a)) - int(log2(b)))

        while a != b:
            a >>= 1
            b >>= 1

        print(10 * a)


if __name__ == '__main__':
    main()
