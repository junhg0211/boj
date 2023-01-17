for _ in range(int(input())):
    print(' '.join(map(lambda x: ''.join(reversed(x)), input().split())))
