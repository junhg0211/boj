from collections import deque

testcase_count = int(input())

for _ in range(testcase_count):
    length = int(input())
    numbers = deque(map(int, input().split()))
    lrs = deque(input())

    while lrs and lrs[0] == "R":
        lrs.popleft()
        numbers.popleft()

    while lrs and lrs[-1] == "L":
        lrs.pop()
        numbers.pop()

    print(sum(numbers))
