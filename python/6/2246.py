def main():
    condos = list()

    count = int(input())
    for _ in range(count):
        condos.append(tuple(map(int, input().split())))

    result = 0
    for i, condo in enumerate(condos):
        is_candidate = True
        for j, c in enumerate(condos):
            if i == j:
                continue

            if condo[0] > c[0] and condo[1] >= c[1]:
                is_candidate = False
                break
            if condo[1] > c[1] and condo[0] >= c[0]:
                is_candidate = False
                break

        if is_candidate:
            result += 1

    print(result)


if __name__ == "__main__":
    main()
