DIVISOR = 1000000007


def main():
    count = int(input())
    numbers = list(map(int, input().split()))

    sum_ = 0
    result = 0
    for number in numbers:
        result = (result + sum_ * number) % DIVISOR

        sum_ = (sum_ + number) % DIVISOR

    print(result)


if __name__ == '__main__':
    main()
