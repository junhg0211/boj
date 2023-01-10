def tick():
    lt, wt, le, we = map(int, input().split())
    at = lt * wt
    ae = le * we

    if at > ae:
        print('TelecomParisTech')
    elif at < ae:
        print('Eurecom')
    else:
        print('Tie')


def main():
    for _ in range(int(input())):
        tick()


if __name__ == '__main__':
    main()
