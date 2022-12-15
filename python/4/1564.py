def main():
    n = int(input())

    factorial = 1
    for i in range(2, n+1):
        factorial *= i

        while factorial % 10 == 0:
            factorial //= 10

        factorial %= 10**12

    print(str(factorial)[-5:])


if __name__ == '__main__':
    main()
