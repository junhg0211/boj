def get_sequence(length):
    for i in range(0, length):
        for j in range(1, length):
            if i >= j:
                continue

            yield (i, j)


def main():
    count = int(input())

    cards = tuple(map(int, input().split()))
    scores = [0 for _ in range(count)]
    for i, j in get_sequence(count):
        if cards[i] % cards[j] == 0:
            scores[j] += 1
            scores[i] -= 1
        elif cards[j] % cards[i] == 0:
            scores[i] += 1
            scores[j] -= 1

    print(' '.join(map(str, scores)))


if __name__ == '__main__':
    main()
