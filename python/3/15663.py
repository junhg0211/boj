numbers = None


def get_sequence(length: int, exclude: tuple = tuple()):
    if length == 1:
        for i in range(len(numbers)):
            if i in exclude:
                continue
            yield str(numbers[i]) + ' '

    for i in range(len(numbers)):
        if i in exclude:
            continue
        for sequence in get_sequence(length-1, exclude + (i,)):
            yield str(numbers[i]) + ' ' + sequence


def main():
    global numbers

    _, length = map(int, input().split())
    numbers = sorted(map(int, input().split()))

    been = set()

    for sequence in get_sequence(length):
        if sequence in been:
            continue

        print(sequence)
        been.add(sequence)


if __name__ == '__main__':
    main()
