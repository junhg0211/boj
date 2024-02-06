max_ = 0
index = -1

for i in range(9):
    number = int(input())

    if max_ < number:
        max_ = number
        index = i

print(max_)
print(index + 1)

