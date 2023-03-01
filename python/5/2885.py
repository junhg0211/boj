from math import log2, ceil


def get_first_bit_position(number: int):
    result = 0
    while number & 1 == 0:
        number >>= 1
        result += 1
    return result


def main():
    k = int(input())

    print(size := 2**ceil(log2(k)), end=' ')
    print(int(log2(size) - get_first_bit_position(k)))


if __name__ == '__main__':
    main()
