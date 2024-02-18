from itertools import combinations


def main():
    numbers = list(map(int, input().split()))

    result = 0
    for combination in combinations(numbers, 2):
        sidea = min(combination)

        new_numbers = list(numbers)
        for number in combination:
            new_numbers.remove(number)
        sideb = min(new_numbers)

        area = sidea * sideb
        result = max(area, result)

    print(result)


if __name__ == '__main__':
    main()
