count = int(input())

for _ in range(count):
    number, string = input().split()
    number = int(number)

    result = ''
    for character in string:
        result += character * number
    print(result)

