def main():
    count = int(input())
    life = tuple(map(int, input().split()))
    joy = tuple(map(int, input().split()))
    people = sorted(map(lambda x: (life[x], joy[x]), range(count)))

    hp = 100
    dp = 0
    for person in people:
        if hp <= person[0]:
            break

        hp -= person[0]
        dp += person[1]

    print(dp)


if __name__ == '__main__':
    main()
