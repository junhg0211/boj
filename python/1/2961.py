from math import inf


def calculate_difference(combination: int, foods: list):
    sour = 1
    bitter = 0

    i = 0
    while combination:
        if combination & 1:
            s, b = foods[i]
            sour *= s
            bitter += b
        i += 1
        combination >>= 1

    return abs(sour - bitter)


def main():
    count = int(input())
    foods = list()

    for _ in range(count):
        foods.append(tuple(map(int, input().split())))

    difference = inf
    for i in range(1, 2**count):
        difference = min(difference, calculate_difference(i, foods))

    print(difference)


if __name__ == '__main__':
    main()
