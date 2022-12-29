def main():
    length = int(input())

    if length <= 2:
        print(length)
        return

    a = 1
    b = 2
    for _ in range(length - 2):
        a, b = b, (a + b) % 15746

    print(b)


if __name__ == '__main__':
    main()
