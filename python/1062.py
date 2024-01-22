from sys import stdin
from itertools import combinations

input = stdin.readline


def main():
    word_count, letter_count = map(int, input().split())
    words = list()
    alphabets = set()
    for _ in range(word_count):
        word = ''.join(set(input().rstrip()))
        words.append(word)
        alphabets = alphabets.union(set(word))

    result = 0
    for combination in combinations(alphabets, letter_count):
        count = 0
        for word in words:
            no = False
            for letter in word:
                if letter not in combination:
                    no = True
                    break
            if no:
                continue
            count += 1

        result = max(count, result)

    print(result)


if __name__ == '__main__':
    main()
