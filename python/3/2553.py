def main():
    a = int(input())

    factorial = 1
    for i in range(2, a+1):
        while i % 10 == 0:
            i //= 10

        factorial *= i
        while factorial % 10 == 0:
            factorial //= 10
        factorial %= 1000000

    while factorial % 10 == 0:
        factorial //= 10
    print(factorial % 10)


if __name__ == '__main__':
    main()
