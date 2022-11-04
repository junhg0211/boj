def contains(value, bottom, top):
    return bottom <= value <= top


routes = list()

for _ in range(int(input())):
    s, e, c = map(int, input().split())
    for route in routes:
        if contains(s, route[0], route[1]) or contains(e, route[0], route[1]) \
                or contains(route[0], s, e) or contains(route[1], s, e):
            routes.remove(route)
            s = min(route[0], s)
            e = max(route[1], e)
            c = min(route[2], c)
    routes.append([s, e, c])

print(len(routes))
for route in sorted(routes, key=lambda x: x[0]):
    print(*route)

