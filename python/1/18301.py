from math import floor


def main():
    n1, n2, n12 = map(int, input().split())

    n = floor((n1+1) * (n2+1) / (n12+1) - 1)

    print(n)


if __name__ == '__main__':
    main()
