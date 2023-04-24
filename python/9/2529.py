relation = None


def get_sequences(numbers: tuple, depth: int = 0):
    possibles = range(10)
    possibles = filter(lambda x: x not in numbers, possibles)

    if depth > 0:
        if relation[depth-1] == '<':
            possibles = filter(lambda x: numbers[depth-1] < x, possibles)
        else:
            possibles = filter(lambda x: numbers[depth-1] > x, possibles)

    if len(relation) == depth:
        for possible in possibles:
            yield str(possible)
        return

    for possible in possibles:
        for sequence in get_sequences(numbers + (possible,), depth+1):
            yield str(possible) + sequence


def main():
    global relation

    count = int(input())
    relation = input().replace(' ', '')

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
