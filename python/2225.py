from functools import cache


@cache
def factorial(number: int):
    if number == 1:
        return 1

    return number * factorial(number-1)


def get_combinations(target: int, count: int, start: int = 0):
    if count == 1:
        return [[target]]

    result = list()
    for i in range(start, target):
        for combination in get_combinations(target-i, count-1, i):
            if i > combination[0]:
                continue
            result.append([i] + combination)
    return result


def main():
    target, count = map(int, input().split())

    result = 0

    for combination in get_combinations(target, count):
        counts = dict()
        for number in combination:
            if number in counts:
                counts[number] += 1
            else:
                counts[number] = 1

        part = factorial(count)
        for value in counts.values():
            part //= factorial(value)
        result += part

    print(result % 10**9)


if __name__ == '__main__':
    main()
