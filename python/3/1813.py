from sys import stdin

input = stdin.readline


def main():
    input()

    counts = dict()
    for number in map(int, input().split()):
        if number in counts:
            counts[number] += 1
        else:
            counts[number] = 1

    true_number = 0
    for key, value in counts.items():
        if key == value and key > true_number:
            true_number = key

    if 0 in counts and true_number == 0:
        print(-1)
        return

    print(true_number)

if __name__ == '__main__':
    main()
