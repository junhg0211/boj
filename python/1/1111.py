def get_possibles(a, b):
    possibles = set()
    for i in range(-200, 201):
        n = b - a * i

        possibles.add((i, n))
    return possibles


def main():
    count = int(input())
    numbers = list(map(int, input().split()))

    if count == 1:
        print("A")
        return

    possibles = get_possibles(numbers[0], numbers[1])
    for i in range(count - 2):
        new_possibles = get_possibles(numbers[i + 1], numbers[i + 2])

        possibles = possibles & new_possibles

    candidates = set()
    for a, b in possibles:
        candidates.add(numbers[-1] * a + b)

    if len(candidates) == 1:
        print(candidates.pop())
    elif len(candidates) > 1:
        print("A")
    else:
        print("B")


if __name__ == "__main__":
    main()
