def is_simillar(word1, word2):
    mapping_table = dict()
    value_been = set()
    for i in range(len(word1)):
        letter = word1[i]
        target_letter = word2[i]

        if letter in mapping_table:
            if target_letter != mapping_table[letter]:
                return False
        else:
            if target_letter in value_been:
                return False

            mapping_table[letter] = target_letter
            value_been.add(target_letter)

    return True


def main():
    count = int(input())
    words = [input() for _ in range(count)]

    result = 0
    for i in range(count - 1):
        for j in range(i+1, count):
            simillar = is_simillar(words[i], words[j])
            if simillar:
                result += 1

    print(result)


if __name__ == '__main__':
    main()
