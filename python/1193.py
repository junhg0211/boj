from math import sqrt


def main():
    n = int(input())

    a = int((-1 + sqrt(1 + 8*(n-1))) / 2) + 1
    b = a - (a*(a+1)//2 - n)
    if a % 2 == 0:
        b = a + 1 - b
    q = a + 1 - b

    print(q, b, sep='/')


if __name__ == '__main__':
    main()
