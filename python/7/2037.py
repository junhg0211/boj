delays = [1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 4, 1, 2, 3, 1, 2, 3, 4]
keys = [2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 5, 6, 6, 6, 7, 7, 7, 7, 8, 8, 8, 9, 9, 9, 9]


def main():
    p, w = map(int, input().split())

    time = 0
    previous_key = 0
    for letter in input():
        if letter == " ":
            previous_key = 1
            time += p
            continue

        key = keys[ord(letter) - ord("A")]
        if previous_key == key:
            time += w

        delay = delays[ord(letter) - ord("A")]
        time += delay * p

        previous_key = key

    print(time)


if __name__ == "__main__":
    main()
