size = int(input())

print(' ' * (size - 1) + '*')
for i in range(size - 2):
    print(' ' * (size - i - 2) + '*' + ' ' * (i * 2 + 1) + '*')
if size != 1:
    print('*' * (size * 2 - 1))
