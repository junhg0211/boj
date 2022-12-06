from math import inf


def main():
    a, b = input().split()

    min_difference = inf
    for i in range(len(b) - len(a) + 1):
        difference = 0
        for j in range(len(a)):
            if a[j] != b[i+j]:
                difference += 1
        min_difference = min(min_difference, difference)

    print(min_difference)


if __name__ == '__main__':
    main()

