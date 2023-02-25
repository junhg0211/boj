from math import inf


def main():
    a = int(ar := input())
    b = int(br := input())

    short_length = inf
    for i in range(-len(br), len(ar)+1):
        ap = a * max(-i, 1)
        bp = b * max(i, 0)
        if '3' not in str(ap + bp):
            short_length = min(max(len(str(ap)), len(str(bp))), short_length)

    print(short_length)


if __name__ == '__main__':
    main()
