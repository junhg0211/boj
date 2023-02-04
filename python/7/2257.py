from copy import copy


def get_list(now, path):
    path = copy(path)
    while path:
        now = now[path.pop(0)]
    return now


def tokenize(formula: str):
    tokens = list()

    path = list()
    for letter in formula:
        if letter == '(':
            the_list = get_list(tokens, path)
            path.append(len(the_list))
            the_list.append(list())
            continue

        if letter == ')':
            path.pop()
            continue

        the_list = get_list(tokens, path)
        the_list.append(letter)

    return tokens


def get_weight(tokens, recursion=0):
    if tokens == 'C':
        return 12
    if tokens == 'H':
        return 1
    if tokens == 'O':
        return 16

    result = 0
    weight = 0
    for token in tokens:
        try:
            multiply = int(token)
        except ValueError:
            result += weight
            weight = get_weight(token, recursion+1)
        except TypeError:
            result += weight
            weight = get_weight(token, recursion+1)
        else:
            result += weight * multiply
            weight = 0

    if weight:
        result += weight

    return result


def main():
    formula = input()

    tokens = tokenize(formula)
    print(get_weight(tokens))


if __name__ == '__main__':
    main()
