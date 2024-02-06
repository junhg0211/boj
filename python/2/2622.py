def main():
    number = int(input())

    result = 0
    for i in range(1, number):
        for j in range(1, i+1):
            k = number - i - j

            if not (k <= j):
                continue
            if not (i < j + k):
                continue

            result += 1

    print(result)


if __name__ == '__main__':
    main()
