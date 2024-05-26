def main():
    condos_by_d = list()
    condos_by_c = list()

    count = int(input())
    for i in range(count):
        wow = tuple(map(int, input().split()))
        wow += (i,)
        condos_by_d.append(wow)
        condos_by_c.append(wow)

    condos_by_d.sort(key=lambda x: x[0])
    condos_by_c.sort(key=lambda x: x[1])

    goods = set(condos_by_d)

    for i in range(count):
        condo = condos_by_d[i]
        for j in range(i):
            c = condos_by_d[j]

            if c[0] == condo[0]:
                break
            if c[1] <= condo[1]:
                goods.remove(condo)
                break

    for condo in list(goods):
        for j in range(condos_by_c.index(condo)):
            c = condos_by_c[j]

            if c[1] == condo[1]:
                break
            if c[0] <= condo[0]:
                goods.remove(condo)
                break

    print(len(goods))


if __name__ == "__main__":
    main()
