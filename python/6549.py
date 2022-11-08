from sys import stdin

input = stdin.readline


def handle(count: int, numbers: list[int]):
    lowers = [(0, 0)]

    max_area = 0
    for i, number in enumerate(numbers):
        for j, (_, height) in enumerate(lowers):
            if not j or height < number:
                continue

            if height == number:
                lowers = lowers[:j+1]
            else:  # elif height > number
                lowers = lowers[:j]

            break

        lowers.append((i, number))

        ipp = i+1
        for position, height in reversed(lowers):
            new_area = (ipp - position) * (height if len(lowers) > 2 else number)
            max_area = max(max_area, new_area)
        print(max_area, lowers, i)

    return max_area

def main():
    results = list()
    while True:
        numbers = map(int, input().split())

        count = next(numbers)
        if not count:
            break

        results.append(str(handle(count, numbers)))

    print('\n'.join(results))


if __name__ == '__main__':
    main()
