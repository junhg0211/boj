count, jump = map(int, input().split())

people = list(range(1, count+1))

print('<', end='')

pointer = 0
while people:
    pointer += jump - 1
    pointer %= len(people)
    print(people[pointer], end=', ' if len(people) - 1 else '')
    people.pop(pointer)

print('>')
