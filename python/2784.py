from copy import copy


def permutations(values):
    if len(values) == 1:
        return [values]

    results = list()
    for value in values:
        excluded = copy(values)
        excluded.remove(value)
        for permutation in permutations(excluded):
            results.append([value] + permutation)

    return results


def is_valid(combination, words):
    words_in_combination = [
        combination[:3],
        combination[3:6],
        combination[6:],
        combination[0] + combination[3] + combination[6],
        combination[1] + combination[4] + combination[7],
        combination[2] + combination[5] + combination[8]
    ]

    for word in words:
        if word in words_in_combination:
            words_in_combination.remove(word)
        else:
            return False
    return True


def main():
    words = [input() for _ in range(6)]
    result = 0
    for combination in permutations(list(range(6))):
        new_words = ''.join(map(lambda x: words[x], combination[:3]))
        if is_valid(new_words, words):
            if result == 0:
                result = new_words
            else:
                result = min(new_words, result)
    print(result)


if __name__ == '__main__':
    main()

