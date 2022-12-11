count = int(input())

for _ in range(count):
    answer_intervals = input().split('X')

    score = 0
    for answer_interval in answer_intervals:
        n = len(answer_interval)
        score += n * (n + 1) // 2

    print(score)

'''
count = int(input())

for _ in range(count):
    answers = input()

    score = 0
    strike = 0

    for answer in answers:
        if answer == 'O':
            strike += 1
            score += strike
        else:
            strike = 0

    print(score)
'''
