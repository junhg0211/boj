def main():
    count = int(input())

    for _ in range(count):
        a, b = input().split()
        print(b * int(a))


if __name__ == '__main__':
    main()
