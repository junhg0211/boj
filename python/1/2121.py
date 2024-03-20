def main():
    point_count = int(input())
    width, height = map(int, input().split())

    points = set()
    for _ in range(point_count):
        x, y = map(int, input().split())

        points.add((x, y))

    rects = set()
    for point in points:
        x, y = point
        u = (x, y - height)
        d = (x, y + height)
        r = (x + width, y)
        l = (x - width, y)
        ur = (x + width, y - height)
        ul = (x - width, y - height)
        dr = (x + width, y + height)
        dl = (x - width, y + height)

        rect = (u, ur, point, r)
        if u in points and ur in points and r in points and rect not in rects:
            rects.add(rect)

        rect = (ul, u, l, point)
        if u in points and ul in points and l in points and rect not in rects:
            rects.add(rect)

        rect = (l, point, dl, d)
        if d in points and dl in points and l in points and rect not in rects:
            rects.add(rect)

        rect = (point, r, d, dr)
        if d in points and dr in points and r in points and rect not in rects:
            rects.add(rect)

    print(len(rects))


if __name__ == '__main__':
    main()
