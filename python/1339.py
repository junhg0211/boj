from itertools import permutations


def main():
    count = int(input())

    numbers = [input() for _ in range(count)]
    max_length = max(map(len, numbers))
    for i, number in enumerate(numbers):
        numbers[i] = "0" * (max_length - len(number)) + number

    number_map = dict()
    next_number = 9
    for i in range(max_length):
        for number in numbers:
            if number[i] == "0":
                continue

            if number[i] in number_map:
                continue

            number_map[number[i]] = str(next_number)
            next_number -= 1

    result = 0
    for i, number in enumerate(numbers):
        for key, value in number_map.items():
            number = number.replace(key, value)

        result += int(number)

    print(result)


if __name__ == "__main__":
    main()
