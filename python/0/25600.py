from math import inf


def get_score(a, b, c):
    score = a * (b + c)
    if a == b + c:
        score *= 2
    return score


def main():
    max_score = -inf
    for _ in range(int(input())):
        a, b, c = map(int, input().split())
        score = get_score(a, b, c)

        max_score = max(score, max_score)

    print(max_score)


if __name__ == '__main__':
    main()
