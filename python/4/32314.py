def main():
    a = int(input())
    w, v = map(int, input().split())

    if a * v > w:
        print(0)
    else:
        print(1)


if __name__ == '__main__':
    main()
