n = int(input())

for i in range(1, n):
    if n == i+sum(map(int, str(i))):
        print(i)
        exit()

print(0)
