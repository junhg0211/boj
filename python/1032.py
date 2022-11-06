count = int(input())

a = list(input())
for _ in range(count-1):
    for i, letter in enumerate(input()):
        if letter != a[i] or a[i] == '?':
            a[i] = '?'

print(''.join(a))
