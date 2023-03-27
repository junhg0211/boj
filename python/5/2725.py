from sys import stdin

input = stdin.readline


def gcd(a, b):
    if a > b:
        return gcd(b, a)

    while True:
        a, b = b%a, a

        if a == 0:
            return b


def main():
    count = int(input())
    numbers = [int(input()) for _ in range(count)]
    number_set = set(numbers)

    result = {1: 3}
    sum_ = 3
    for i in range(2, max(numbers) + 1):
        for j in range(1, i):
            if gcd(i, j) == 1:
                sum_ += 2

        if i in number_set:
            result[i] = sum_

    for number in numbers:
        print(result[number])


if __name__ == '__main__':
    main()
