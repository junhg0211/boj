NAMES = 'zero one two three four five six seven eight nine'.split()


def convert(number: int) -> list[str]:
    result = list()
    for letter in str(number):
        result.append(NAMES[int(letter)])
    return result


def main():
    a, b = map(int, input().split())

    converteds = list()
    for i in range(a, b+1):
        converteds.append((convert(i), i))

    numbers = map(lambda x: x[1], sorted(converteds, key=lambda x: x[0]))
    for i, number in enumerate(numbers):
        print(number, end=' ')
        if i % 10 == 9:
            print()


if __name__ == '__main__':
    main()
