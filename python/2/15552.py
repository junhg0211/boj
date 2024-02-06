from sys import stdout, stdin
list(stdout.write(f'{sum(map(int, stdin.readline().split()))}\n') for _ in range(int(input())))
