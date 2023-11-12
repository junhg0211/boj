count = int(input())
flowers = list(map(int, input().split()))

for _ in range(int(input())):
    message = list(map(int, input().split()))

    if message[0] == 1:
        print(flowers[message[1]-1:message[2]].count(message[3]))
    else:
        for i in range(message[1]-1, message[2]):
            flowers[i] = 0
