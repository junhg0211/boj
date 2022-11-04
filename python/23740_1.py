number = int(input())
routes = list()
for _ in range(number):
    routes.append(list(map(int, input().split())))


def contains(value, bottom, top):
    return bottom <= value <= top;


def is_duplicate(route1, route2):
    return contains(route1[0], route2[0], route2[1]) or contains(route1[1], route2[0], route2[1]) or \
            contains(route2[0], route1[0], route1[1]) or contains(route2[1], route1[0], route1[1])


def merge(route1, route2):
    return [min(route1[0], route2[0]), max(route1[1], route2[1]), min(route1[2], route2[2])]


i, offset = 0, 0
while i < number - offset:
    j = i + 1
    while j < number - offset:
        if is_duplicate(routes[i], routes[j]):
            routes[i] = merge(routes[i], routes.pop(j))
            offset += 1
            j -= 1
        j += 1
    i += 1


print(len(routes))
for route in sorted(routes, key=lambda x: x[0]):
    print(*route)

