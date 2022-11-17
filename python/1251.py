def words(word: str):
    length = len(word)
    for i in range(1, length - 1):
        for j in range(i+1, length):
            result = word[:i][::-1] + word[i:j][::-1] + word[j:][::-1]
            yield result


def main():
    word = input()

    first = None
    for word in words(word):
        if first is None or word < first:
            first = word

    print(first)


if __name__ == '__main__':
    main()
