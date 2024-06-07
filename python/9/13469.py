from math import ceil, sqrt


def main():
    original_number = int(input())
    number = original_number

    divisors = set()

    for i in range(2, ceil(sqrt(number)) + 1):
        while number % i == 0:
            number //= i
            divisors.add(i)

    if number != 1:
        divisors.add(number)
        number = 1

    if len(divisors) == 1:
        print('yes')
    else:
        print('no')


if __name__ == "__main__":
    main()
