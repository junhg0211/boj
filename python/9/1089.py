font = '''\
###...#.###.###.#.#.###.###.###.###.###
#.#...#...#...#.#.#.#...#.....#.#.#.#.#
#.#...#.###.###.###.###.###...#.###.###
#.#...#.#.....#...#...#.#.#...#.#.#...#
###...#.###.###...#.###.###...#.###.###\
'''.split()


def parse_font(strings, x_offset):
    result = set()
    for j in range(5):
        for k in range(3):
            if strings[j][x_offset + k] == '#':
                result.add((j, k))
    return result


def main():
    # -- parse font
    font_set = dict()
    for i in range(10):
        font_set[i] = parse_font(font, 4*i)

    # -- get variables
    length = int(input())

    lines = list()
    for _ in range(5):
        lines.append(input())

    result = 0
    for i in range(length):
        weight = 10**(length-i-1)
        parsed = parse_font(lines, 4*i)
        possibles = set()
        for j in range(10):
            if len(parsed - font_set[j]):
                continue
            possibles.add(j)

        if not possibles:
            print('-1')
            return

        result += sum(possibles) / len(possibles) * weight

    print(result)


if __name__ == '__main__':
    main()
