number = int(input())
points = set()
routes = list()
for i in range(number):
    routes.append(list(map(int, input().split())))
    points.add(routes[-1][0])
    points.add(routes[-1][1])


def routes_containing_point(point):
    result = list()
    start_min = 1000000000
    end_max = 0
    fee_min = 1000000000
    for route in routes:
        if route[0] <= point <= route[1]:
            result.append(route)
            start_min = min(route[0], start_min)
            end_max = max(route[1], end_max)
            fee_min = min(route[2], fee_min)
    return result, start_min, end_max, fee_min


results = list()
previous_intersects = list()

for point in sorted(points):
    containing_routes, start_min, end_max, fee_min = routes_containing_point(point)
    if previous_intersects and results[-1][1] >= point:
        results[-1][0] = min(start_min, results[-1][0])
        results[-1][1] = max(end_max, results[-1][1])
        results[-1][2] = min(fee_min, results[-1][2])
    else:
        results.append([start_min, end_max, fee_min])
    previous_intersects = containing_routes


print(len(results))
for result in results:
    print(result[0], result[1], result[2])

