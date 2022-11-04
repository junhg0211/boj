numbers = input().split()

for i, number in enumerate(numbers):
    numbers[i] = int(number)


ascending = True
descending = True

for i in range(len(numbers) - 1):
    this_number = numbers[i]
    next_number = numbers[i+1]

    if this_number >= next_number:
        ascending = False
    if this_number <= next_number:
        descending = False

if ascending:
    print('ascending')
elif descending:
    print('descending')
else:
    print('mixed')

