iterations = int(input())


def get_line(i):
    print(' ' * i, end='')
    if i == 0:
        print('*' * iterations, end='')
    else:
        print('*' + ' ' * (iterations - 2) + '*', end='')
    print(' ' * ((iterations - i - 1) * 2 - 1), end='')
    if i == 0:
        print('*' * iterations, end='')
    else:
        print('*' + ' ' * (iterations - 2) + '*', end='')


for line in range(iterations - 1):
    get_line(line)
    print()
print(' ' * (iterations - 1) + ('*' + ' ' * (iterations - 2)) * 2 + '*')
for line in range(iterations - 1):
    get_line(iterations - line - 2)
    print()
