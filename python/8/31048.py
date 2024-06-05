from math import factorial


def tick():
    print(factorial(int(input())) % 10)


def main():
    count = int(input())

    for _ in range(count):
        tick()


if __name__ == '__main__':
    main()
