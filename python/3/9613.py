def gcd(a, b):
    if a < b:
        return gcd(b, a)

    while b > 0:
        a, b = b, a%b

    return a


def tick():
    numbers = map(int, input().split())
    count = next(numbers)
    numbers = list(numbers)

    sum_ = 0
    for i in range(count):
        for j in range(i+1, count):
            sum_ += gcd(numbers[i], numbers[j])
    print(sum_)


def main():
    testcase_count = int(input())

    for _ in range(testcase_count):
        tick()


if __name__ == '__main__':
    main()
