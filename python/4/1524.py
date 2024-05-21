def tick():
    input()

    _, _ = map(int, input().split())
    s_hps = sorted(map(int, input().split()))
    b_hps = sorted(map(int, input().split()))

    while True:
        if len(s_hps) == 0:
            print("B")
            break
        elif len(b_hps) == 0:
            print("S")
            break

        s_min = s_hps[0]
        b_min = b_hps[0]

        if s_min < b_min:
            s_hps.pop(0)
        elif b_min < s_min:
            b_hps.pop(0)
        else:
            b_hps.pop(0)


def main():
    testcase_count = int(input())

    for _ in range(testcase_count):
        tick()


if __name__ == "__main__":
    main()
