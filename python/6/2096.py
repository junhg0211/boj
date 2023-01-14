from copy import copy


def main():
    line_count = int(input())-1

    min_numbers = tuple(map(int, input().split()))
    max_numbers = min_numbers

    for _ in range(line_count):
        numbers = tuple(map(int, input().split()))

        min_numbers = (
            numbers[0] + min(min_numbers[:2]),
            numbers[1] + min(min_numbers),
            numbers[2] + min(min_numbers[1:])
        )

        max_numbers = (
            numbers[0] + max(max_numbers[:2]),
            numbers[1] + max(max_numbers),
            numbers[2] + max(max_numbers[1:])
        )

    print(max(max_numbers), min(min_numbers))


if __name__ == '__main__':
    main()
