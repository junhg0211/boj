def get_segments():
    segments = [0, 0, 0, 0]

    line = input()
    segments[0] += int(line[ 1] == '#') * 0b1000000
    segments[1] += int(line[ 5] == '#') * 0b1000000
    segments[2] += int(line[ 9] == '#') * 0b1000000
    segments[3] += int(line[13] == '#') * 0b1000000

    line = input()
    segments[0] += int(line[ 0] == '#') * 0b0100000
    segments[1] += int(line[ 4] == '#') * 0b0100000
    segments[2] += int(line[ 8] == '#') * 0b0100000
    segments[3] += int(line[12] == '#') * 0b0100000

    segments[0] += int(line[ 2] == '#') * 0b0010000
    segments[1] += int(line[ 6] == '#') * 0b0010000
    segments[2] += int(line[10] == '#') * 0b0010000
    segments[3] += int(line[14] == '#') * 0b0010000

    line = input()
    segments[0] += int(line[ 1] == '#') * 0b0001000
    segments[1] += int(line[ 5] == '#') * 0b0001000
    segments[2] += int(line[ 9] == '#') * 0b0001000
    segments[3] += int(line[13] == '#') * 0b0001000

    line = input()
    segments[0] += int(line[ 0] == '#') * 0b0000100
    segments[1] += int(line[ 4] == '#') * 0b0000100
    segments[2] += int(line[ 8] == '#') * 0b0000100
    segments[3] += int(line[12] == '#') * 0b0000100

    segments[0] += int(line[ 2] == '#') * 0b0000010
    segments[1] += int(line[ 6] == '#') * 0b0000010
    segments[2] += int(line[10] == '#') * 0b0000010
    segments[3] += int(line[14] == '#') * 0b0000010

    line = input()
    segments[0] += int(line[ 1] == '#') * 0b0000001
    segments[1] += int(line[ 5] == '#') * 0b0000001
    segments[2] += int(line[ 9] == '#') * 0b0000001
    segments[3] += int(line[13] == '#') * 0b0000001

    return segments


def get_nth_digit(segment, n) -> bool:
    return bool((segment >> (6-n)) & 1)


digits = [
    0b1110111, 0b0010010, 0b1011101, 0b1011011, 0b0111010,
    0b1101011, 0b1101111, 0b1110010, 0b1111111, 0b1111011
]


def get_nth_bit(number, n):
    return (number >> n) & 1


def get_possible(digit):
    for i, real_digit in enumerate(digits):
        possible = True
        for j in range(7):
            if get_nth_bit(real_digit, j) == 0 and get_nth_bit(digit, j) == 1:
                possible = False
                break
        if possible:
            return i


def main():
    segments = get_segments()

    for i in range(4):
        print(get_possible(segments[i]), end='' if i != 1 else ':')
    print()


if __name__ == '__main__':
    main()
