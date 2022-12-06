def is_differentiable(codes: list[str], length: int) -> bool:
    gots = dict()
    for code in codes:
        code = code[-length:]
        if code in gots:
            return False
        gots[code] = None
    return True


def main():
    codes = [input() for _ in range(int(input()))]

    for i in range(1, len(codes[0])+1):
        if is_differentiable(codes, i):
            print(i)
            return


if __name__ == '__main__':
    main()
