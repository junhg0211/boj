def toss(word: str, i: int) -> str:
    return word[i:] + word[:i]


def main():
    count = int(input())

    words = list()

    for _ in range(count):
        word = input()
        exists = False
        for i in range(len(word)):
            if toss(word, i) in words:
                exists = True
                break
        if not exists:
            words.append(word)

    print(len(words))


if __name__ == '__main__':
    main()
