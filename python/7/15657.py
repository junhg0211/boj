numbers = None


def get_sequence(length: int, start: int = 0):
    if length == 1:
        for i in range(start, len(numbers)):
            yield str(numbers[i]) + ' '
        return

    for i in range(start, len(numbers)):
        for sequence in get_sequence(length-1, i):
            yield str(numbers[i]) + ' ' + sequence


def main():
    global numbers

    count, length = map(int, input().split())
    numbers = sorted(map(int, input().split()))

    for sequence in get_sequence(length):
        print(sequence)

if __name__ == '__main__':
    main()
