from typing import Optional

numbers = None


def get_sequence(count: int, length: int, exclude: Optional[tuple] = None):
    if exclude is None:
        exclude = ()

    if length == 1:
        for i in range(len(numbers)):
            if numbers[i] in exclude:
                continue
            yield str(numbers[i]) + ' '
        return

    for i in range(len(numbers)):
        if numbers[i] in exclude:
            continue
        for sequence in get_sequence(count, length-1, exclude + (numbers[i],)):
            yield str(numbers[i]) + ' ' + sequence


def main():
    global numbers

    count, length = map(int, input().split())
    numbers = sorted(map(int, input().split()))

    for sequence in get_sequence(count, length):
        print(sequence)


if __name__ == '__main__':
    main()
