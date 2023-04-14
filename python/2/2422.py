from sys import stdin

input = stdin.readline


def main():
    total_count, prohibits = map(int, input().split())

    nots = dict()

    for _ in range(prohibits):
        a, b = map(int, input().split())

        if a > b:
            a, b = b, a

        if a in nots:
            nots[a].add(b)
        else:
            nots[a] = {b}

    count = 0

    for i in range(1, total_count+1):
        for j in range(i+1, total_count+1):
            if j in nots.get(i, ()):
                continue
            for k in range(j+1, total_count+1):
                if k in nots.get(j, ()) or k in nots.get(i, ()):
                    continue

                count += 1

    print(count)


if __name__ == '__main__':
    main()
