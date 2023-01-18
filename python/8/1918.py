from math import inf


def get_index(iterable, values):
    min_index = inf
    for value in values:
        try:
            min_index = min(min_index, iterable.index(value))
        except ValueError:
            pass
    return min_index if min_index != inf else -1


def parse(tokens):
    if len(tokens) == 1 and isinstance(tokens[0], list):
        return parse(tokens[0])

    if isinstance(tokens, str):
        return tokens

    while (i := get_index(tokens, ('*', '/'))) != -1:
        tokens[i-1:i+2] = [parse(tokens[i-1]) + parse(tokens[i+1]) + tokens[i]]

    while (i := get_index(tokens, ('+', '-'))) != -1:
        tokens[i-1:i+2] = [parse(tokens[i-1]) + parse(tokens[i+1]) + tokens[i]]

    return ''.join(tokens)


def main():
    tokens = list()
    stack = 0
    for letter in input():
        ts = tokens
        for _ in range(stack):
            ts = ts[-1]

        if 'A' <= letter <= 'Z' or letter in '+-*/':
            ts.append(letter)
            continue

        if letter == '(':
            stack += 1
            ts.append(list())
            continue

        if letter == ')':
            stack -= 1
            continue

    print(parse(tokens))


if __name__ == '__main__':
    main()
