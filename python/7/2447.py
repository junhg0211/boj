from sys import stdout
from math import log


def is_star(x, y, iteration):
    if x % 3 == y % 3 == 1:
        return False

    if not iteration:
        return True
    else:
        return is_star(x // 3, y // 3, iteration - 1)


size = int(input())
iterations = int(log(size, 3))

result = ''

for y in range(size):
    for x in range(size):
        result += '*' if is_star(x, y, iterations) else ' '
    print(result)
    result = ''

stdout.write(result)
