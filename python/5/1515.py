from collections import deque


def main():
    letters = deque(input())

    i = 1
    while letters:
        for letter in str(i):
            if not letters:
                break
            if letters[0] == letter:
                letters.popleft()

        i += 1

    print(i-1)


if __name__ == '__main__':
    main()
