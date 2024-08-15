testcase_count = int(input())

for _ in range(testcase_count):
    no = False

    count = int(input())
    seats = [False for _ in range(count)]
    for i, seat in enumerate(map(int, input().split())):
        seat -= 1

        seats[seat] = True

        if i == 0:
            continue

        if not (seat < count - 1 and seats[seat + 1] or seat > 0 and seats[seat - 1]):
            no = True
            break

    if no:
        print("NO")
    else:
        print("YES")
