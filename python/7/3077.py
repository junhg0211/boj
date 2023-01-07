def main():
    count = int(input())
    answer = dict()

    for i, word in enumerate(input().split()):
        answer[word] = i

    words = input().split()

    total_score = 0
    score = 0
    for i in range(count-1):
        word1 = words[i]
        for j in range(i+1, count):
            total_score += 1
            word2 = words[j]
            if answer[word1] < answer[word2]:
                score += 1

    print(score, total_score, sep='/')


if __name__ == '__main__':
    main()
