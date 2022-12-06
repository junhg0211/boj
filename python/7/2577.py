a = int(input())
b = int(input())
c = int(input())

number = str(a * b * c)

frequency = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

for letter in number:
    frequency[int(letter)] += 1

for count in frequency:
    print(count)

'''
number = a * b * c

frequency = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
while number:
    wkfltn = number % 10
    frequency[wkfltn] += 1

    number //= 10

for count in frequency:
    print(count)
    '''

