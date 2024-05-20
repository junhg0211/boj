numbers = 0

for number in map(int, input().split()):
    if not ((1 << number) & numbers):
        print(number, end=" ")

    numbers |= 1 << number
print()
