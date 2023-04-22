from collections import deque


def letters(string: str):
    queue = deque(())

    yield (string[0], 0)
    skip = False
    for i, letter in enumerate(string):
        if i == 0:
            continue
        if skip:
            skip = False
            continue

        if letter == ' ':
            yield (string[i+1], i+1)
            skip = True
            continue

        queue.append((letter, i))

    for letter in queue:
        yield letter


def main():
    count = int(input())

    shortcuts = set()
    for word in (input() for _ in range(count)):
        fail = True
        for letter, i in letters(word):
            if letter.lower() in shortcuts:
                continue

            print(f'{word[:i]}[{letter}]{word[i+1:]}')
            shortcuts.add(letter.lower())
            fail = False
            break

        if fail:
            print(word)


if __name__ == '__main__':
    main()
