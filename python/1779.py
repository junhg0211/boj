def main():
    abbreviations = {}
    abbreviations1 = {}
    n1, n2 = map(int, input().split())
    for _ in range(n1):
        a, b = input()[1:-1].split('" -> "')
        abbreviations[a] = b
    for _ in range(n2):
        a, b = input()[1:-1].split('" -> "')
        abbreviations1[a] = b

    abbreviations2 = abbreviations1.copy()

    while True:
        try:
            line = input()
        except EOFError:
            break

        if line == '#':
            abbreviations1 = abbreviations2.copy()

        for a, b in abbreviations.items():
            line = line.replace(a, b)

        for a in tuple(abbreviations1.keys()):
            if a not in line:
                continue
            b = abbreviations1.pop(a)
            line = line.replace(a, f'{b} ({a})', 1)

        print(line)


if __name__ == '__main__':
    main()
