def tick():
    for _ in range(int(input())):
        a, b = map(int, input().split())
        print(a + b, a * b)


def main():
    for _ in range(int(input())):
        tick()


if __name__ == '__main__':
    main()
