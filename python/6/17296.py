def main():
    input()

    pressing = False
    presses = 0
    for a_s in map(float, input().split()):
        if a_s % 1 == 0.5:
            if not pressing:
                pressing = True
                presses += 1
            a_s -= 0.5

        if a_s:
            presses += int(a_s)
            pressing = True

    print(presses)


if __name__ == '__main__':
    main()
