size = int(input())

print(' ' * (size - 1) + '*')
for i in range(size - 1):
    print(' ' * (size - i - 2) + '*' + ' ' * (i * 2 + 1) + '*')
