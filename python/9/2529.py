relation = None


def get_sequences(numbers: tuple):
    possibles = range(10)
    possibles = filter(lambda x: x not in numbers, possibles)

    if len(numbers) > 0:
        if relation[len(numbers)-1] == '<':
            possibles = filter(lambda x: numbers[len(numbers)-1] < x, possibles)
        else:
            possibles = filter(lambda x: numbers[len(numbers)-1] > x, possibles)

    if len(relation) == len(numbers):
        for possible in possibles:
            yield str(possible)
        return

    for possible in possibles:
        for sequence in get_sequences(numbers + (possible,)):
            yield str(possible) + sequence


def main():
    global relation

    count = int(input())
    relation = input().split()

    first = None
    last = None
    for sequence in get_sequences(()):
        if first is None:
            first = sequence
        last = sequence

    print(last)
    print(first)


if __name__ == '__main__':
    main()
