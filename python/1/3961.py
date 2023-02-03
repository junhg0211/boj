from sys import stdin

input = stdin.readline


positions = [
    (1,0), (2,4), (2,2), (1,2), (0,2), (1,3), (1,4),
    (1,5), (0,7), (1,6), (1,7),
    (1,8), (2,6), (2,5), (0,8), (0,9),
    (0,0), (0,3), (1,1), (0,4), (0,6), (2,3),
    (0,1), (2,1), (0,5), (2,0)
]

A = ord('a')


def letter_distance(a, b):
    a = positions[ord(a) - A]
    b = positions[ord(b) - A]
    return abs(a[0] - b[0]) + abs(a[1] - b[1])


def tick():
    word, count = input().split()
    count = int(count)

    words = list()
    for _ in range(count):
        candidate = input().rstrip()
        distance = sum(letter_distance(word[i], candidate[i]) for i in range(len(word)))
        words.append((distance, candidate))
    words.sort()

    for distance, word in words:
        print(word, distance)


def main():
    for _ in range(int(input())):
        tick()


if __name__ == '__main__':
    main()
