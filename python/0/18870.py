from sys import stdin

input = stdin.readline


def get_index(number: int, numbers: list, length: int):
    start = 0
    end = length

    while True:
        anchor = (start + end) // 2
        if number < numbers[anchor]:
            end = anchor
        elif number > numbers[anchor]:
            start = anchor
        else:
            return anchor

    print(numbers, start, end, number)

def main():
    input()

    numbers = tuple(map(int, input().split()))
    sorteds = sorted(set(numbers))
    length = len(sorteds)

    for number in numbers:
        print(get_index(number, sorteds, length), end=' ')


if __name__ == '__main__':
    main()
