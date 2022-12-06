from math import inf

def main():
    input()
    minimum = inf
    maximum = -inf
    for x in input().split():
        x = int(x)
        if x < minimum:
            minimum = x
        if x > maximum:
            maximum = x

    print(maximum * minimum)


if __name__ == '__main__':
    main()
