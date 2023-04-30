from clipboard import copy

number = [106, 117, 110, 104, 103, 48, 50, 49, 49]

result = ''

previous = 0

for letter in number:
    if previous < letter:
        result += '+' * (letter - previous)
    else:
        result += '-' * (previous - letter)
    result += '.'
    previous = letter

copy(result)
print(result)
