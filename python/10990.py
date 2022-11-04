n = int(input())

print(' ' * (n-1) + '*')

for i in range(n-1):
    print(' ' * (n-2-i) + '*' + ' ' * (1 + 2*i) + '*')

