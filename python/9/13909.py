def main():
    n = int(input())

    i = 1
    while True:
        if i*i > n:
            break

        i += 1

    print(i - 1)


if __name__ == '__main__':
    main()
