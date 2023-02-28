def get_sequence(length: int, numbers: list, start: int = 0):
    if length == 1:
        for i in range(start, len(numbers)):
            number = numbers[i]
            yield f'{number} '
        return

    for i in range(start, len(numbers)):
        for sequence in get_sequence(length-1, numbers, i+1):
            number = numbers[i]
            yield f'{number} {sequence}'


def main():
    count, length = map(int, input().split())
    numbers = sorted(map(int, input().split()))

    been = set()

    for sequence in get_sequence(length, numbers):
        if sequence in been:
            continue

        print(sequence)
        been.add(sequence)


if __name__ == '__main__':
    main()
