from collections import deque

testcase_count = int(input())

for _ in range(testcase_count):
    length = int(input())
    numbers = list()
    adds = [0]
    for number in map(int, input().split()):
        numbers.append(number)
        adds.append(adds[-1] + number)
    lrs = deque(input())

    first_ls = list()
    last_rs = list()
    for i, letter in enumerate(lrs):
        if letter == "L":
            first_ls.append(i)
        else:
            last_rs.append(i)

    result = 0
    for i in range(min(len(first_ls), len(last_rs))):
        from_ = first_ls[i]
        to = last_rs[-i - 1]

        if from_ > to:
            break
        result += adds[to + 1] - adds[from_]

    print(result)
