def main():
    scores = [0, 0, 0, 0, 0]
    int(input())
    for i, score in enumerate(map(int, input().split())):
        scores[i] = score

    if scores[0] > scores[2]:
        a = abs(scores[0] - scores[2]) * 508
    else:
        a = abs(scores[0] - scores[2]) * 108

    if scores[1] > scores[3]:
        b = abs(scores[1] - scores[3]) * 212
    else:
        b = abs(scores[1] - scores[3]) * 305

    c = scores[4] * 707

    score = (a + b + c) * 4763

    print(score)


if __name__ == "__main__":
    main()
