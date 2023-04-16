from sys import stdin

input = stdin.readline


def main():
    count = int(input())
    ranking = sorted((tuple(map(int, input().split())) for _ in range(count)),
                     key=lambda x: x[2],
                     reverse=True)

    counts = dict()

    count = 0
    for nation, number, _ in ranking:
        if count >= 3:
            break
        if counts.get(nation, 0) >= 2:
            continue

        counts[nation] = counts.get(nation, 0) + 1
        print(nation, number)
        count += 1


if __name__ == '__main__':
    main()
