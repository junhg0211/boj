n, x = input().split()
n, x = int(n), int(x)

numbers = input().split()
result = ''
for i, number in enumerate(numbers):
    number = int(number)
    if number < x:
        result += f'{number} '

print(result)

