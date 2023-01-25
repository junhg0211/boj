from math import sqrt

cache = dict()


def minimal_number(number: int):
    # print(0, number)

    if number in cache:
        # print(1, number)
        return cache[number]

    root = int(sqrt(number))
    if number / root == root:
        cache[number] = 1
        # print(2, number)
        return 1

    value = 1 + minimal_number(number - root**2)
    cache[number] = value
    # print(3, number)
    return value


def main():
    print(minimal_number(int(input())))


if __name__ == '__main__':
    main()
