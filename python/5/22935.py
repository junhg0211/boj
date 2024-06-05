def tick():
    number = int(input())

    wow = (number - 1) % 28 + 1

    if wow > 15:
        wow = 30 - wow

    print("딸기" if wow & 0b1000 else "V", end="")
    print("딸기" if wow & 0b0100 else "V", end="")
    print("딸기" if wow & 0b0010 else "V", end="")
    print("딸기" if wow & 0b0001 else "V", end="")
    print()


def main():
    count = int(input())

    for _ in range(count):
        tick()


if __name__ == "__main__":
    main()
