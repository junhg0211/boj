life_count, experiment_count = map(int, input().split())

lifes = [[None, None] for _ in range(life_count)]

for _ in range(experiment_count):
    tokens = input().split()
    tokens[0] = int(tokens[0])
    tokens[2] = int(tokens[2])

    if tokens[1] == 'P':
        lifes[tokens[0]-1][0] = tokens[2] == 1
    else:
        lifes[tokens[0]-1][1] = tokens[2] == 1


possible_count = 0
exact_count = 0
for i in range(life_count):
    life = lifes[i]

    if not life[0] and life[0] is not None:
        continue
    if life[1] and life[1] is not None:
        continue

    possible_count += 1
    if life[0] is True and life[1] is False:
        exact_count += 1

print(exact_count, possible_count)

