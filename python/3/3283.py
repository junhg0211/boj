from sys import stdin

input = stdin.readline


def main():
    length = int(input())

    data = dict()

    for _ in range(5):
        for i, letter in enumerate(input()):
            if letter in ('.', 'X'):
                data[i] = letter

    while True:
        nos = set(range(length)) - set(data.keys())
        changed = False

        for unknown in nos:
            # ?AA -> B
            if data.get(unknown+1, '?') == data.get(unknown+2, '?') != '?':
                data[unknown] = 'X' if data[unknown+1] == '.' else '.'
                changed = True
                continue

            # AA? -> B
            if data.get(unknown-2, '?') == data.get(unknown-1, '?') != '?':
                data[unknown] = 'X' if data[unknown-1] == '.' else '.'
                changed = True
                continue

            # A?A -> B
            if data.get(unknown-1, '?') == data.get(unknown+1, '?') != '?':
                data[unknown] = 'X' if data[unknown-1] == '.' else '.'
                changed = True
                continue

        if not changed:
            break

    if len(data.keys()) < length:
        print('UNDETERMINABLE')
        return

    result = list()
    i = 0
    while i < length:
        if length - 1 == i:
            result.append('0')
            break

        if data[i] == data[i+1]:
            result.append('1')
            i += 2
            continue

        result.append('0')
        i += 1

    print(''.join(result))


if __name__ == '__main__':
    main()
