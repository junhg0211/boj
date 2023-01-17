numbers = None


def get_sequence(length, start: int = 0):
    if length == 1:
        for i in range(start, len(numbers)):
            number = numbers[i]
            yield str(number) + ' '
        return

    for i in range(start, len(numbers)):
        number = numbers[i]
        for sequence in get_sequence(length-1, i):
            yield str(number) + ' ' + sequence


def main():
    global numbers

    _, length = map(int, input().split())
    numbers = sorted(set(map(int, input().split())))

    for sequence in get_sequence(length):
        print(sequence)


if __name__ == '__main__':
    '''
    from io import StringIO
    input = StringIO('4 4\n1 1 2 2').readline
    '''

    main()
