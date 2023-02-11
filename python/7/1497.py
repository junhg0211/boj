from math import inf
from sys import stdin

input = stdin.readline


def get_ones(number: int) -> list:
    result = list()
    i = 0
    while number:
        if number & 1:
            result.append(i)
        number >>= 1
        i += 1

    return result


def main():
    guitar_count, song_count = map(int, input().split())

    record = -1
    guitar_data = list()
    for i in range(guitar_count):
        possibles = input().split(' ')[1]
        binary = possibles.replace('Y', '1').replace('N', '0')
        datum = int(binary, 2)

        guitar_data.append(datum)
        if datum > 0:
            record = 0

    if record == -1:
        print(record)
        return

    max_song_count = 0
    that_guitar_count = 0
    for i in range(1 << guitar_count):
        guitar_indexes = get_ones(i)
        guitar_count_now = len(guitar_indexes)

        buffer = 0
        for guitar_datum in map(lambda x: guitar_data[x], guitar_indexes):
            buffer |= guitar_datum
        song_count_now = len(get_ones(buffer))

        if song_count_now > max_song_count:
            that_guitar_count = guitar_count_now
            max_song_count = song_count_now
        elif song_count_now == max_song_count:
            that_guitar_count = min(guitar_count_now, that_guitar_count)

    print(that_guitar_count)


if __name__ == '__main__':
    main()
