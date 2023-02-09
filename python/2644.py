def main():
    parents = list()
    kids = list()

    for i in range(int(input())):
        parents.append(set())
        kids.append(set())

    a, b = map(lambda x: int(x) - 1, input().split())

    for _ in range(int(input())):
        x, y = map(lambda x: int(x) - 1, input().split())

        parents[y].add(x)
        kids[x].add(y)

    been = set()


if __name__ == '__main__':
    main()
