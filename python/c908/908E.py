def main():
    type_count = int(input())
    flower_info = list()
    prob_total = 0
    for _ in range(type_count):
        flower_info.append(list(map(int, input().split())))
        prob_total += flower_info[-1][1]


if __name__ == '__main__':
    main()
