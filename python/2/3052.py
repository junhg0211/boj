numbers = list()

for _ in range(10):
    number = int(input()) % 42

    if number not in numbers:
        numbers.append(number)

print(len(numbers))

