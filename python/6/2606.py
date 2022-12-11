from sys import stdin, stdout

computer_count = int(stdin.readline())  # 1 - n
connections = dict()
for _ in range(int(stdin.readline())):
    connecto, connectee = map(int, stdin.readline().split())

    if connecto in connections:
        connections[connecto].append(connectee)
    else:
        connections[connecto] = [connectee]

    if connectee in connections:
        connections[connectee].append(connecto)
    else:
        connections[connectee] = [connecto]


infections = [1]
queue = connections[1]

while queue:
    connecto = queue.pop(0)

    queue.extend(filter(
        lambda x: x not in infections and x not in queue,
        connections[connecto] if connecto in connections else list()))
    infections.append(connecto)

stdout.write(str(len(infections) - 1))

