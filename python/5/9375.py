items = dict()

for _ in range(int(input())):
    items.clear()

    for _ in range(int(input())):
        _, kind = input().split()

        if kind in items:
            items[kind] += 1
        else:
            items[kind] = 2

    combinations = 1
    for value in items.values():
        combinations *= value
    print(combinations - 1)

