def get_sequence(limit: int, length: int, start: int = 1):
    if length == 1:
        for i in range(start, limit+1):
            yield str(i) + ' '

    for i in range(start, limit+1):
        for sequence in get_sequence(limit, length-1, i+1):
            yield str(i) + ' ' + sequence


def main():
    limit, length = map(int, input().split())

    for sequence in get_sequence(limit, length):
        print(sequence)


if __name__ == '__main__':
    from sys import stdin
    input = stdin.readline

    main()
