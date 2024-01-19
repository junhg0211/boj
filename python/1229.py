from copy import copy
from math import inf

numbers = set()


def get_hexagonal_number(n: int):
    return n * (2 * n - 1)


'''
dp = dict()


def get_result(number):
    i = binary_search(number)

    if numbers[i] == number:
        return 1

    if number in dp:
        return dp[number]

    print(number, i)

    min_ = inf
    while i > 0:
        min_ = min(min_, get_result(number - numbers[i]) + 1)

        i -= 1

    dp[number] = min_
    return min_
'''


'''
dp = dict()


def get_result(number):
    for i in range(1, number+1):
        if i in numbers:
            dp[i] = 1
            continue

        j = 1
        min_ = inf
        while numbers[j] < i:
            min_ = min(min_, 1 + dp[i - numbers[j]])
            j += 1

        dp[i] = min_

    return dp[number]
'''


def get_result(number):
    if number in numbers:
        return 1

    beens = copy(numbers)

    previous_numbers = copy(numbers)
    new_numbers = set()

    epoch = 2
    while True:
        for i in previous_numbers:
            for hn in numbers:  # hn stands for hexagonal number
                new_number = i + hn
                if new_number > number:
                    continue
                if new_number in beens:
                    continue
                print(new_number)

                new_numbers.add(new_number)
                beens.add(new_number)
                if new_number == number:
                    return epoch
        previous_numbers = new_numbers
        new_numbers = set()
        epoch += 1


def main():
    n = int(input())

    i = 1
    while not numbers or max(numbers) < n:
        numbers.add(get_hexagonal_number(i))
        i += 1

    print(get_result(n))


if __name__ == '__main__':
    main()
