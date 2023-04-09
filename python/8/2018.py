def main():
    n = int(input())

    k = 1
    count = 0
    while True:
        a = n - k * (k+1) // 2

        if a >= 0:
            if a % k == 0:
                count += 1
        else:
            break

        k += 1

    print(count)


if __name__ == '__main__':
    main()
