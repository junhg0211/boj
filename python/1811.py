def tick(message: str):
    correct, answer = message.split()

    been = set()
    black, grey, white = 0, 0, 0
    for i, letter in enumerate(answer):
        if letter == correct[i] and i not in been:
            black += 1
            been.add(i)
            continue

        if i > 0 and correct[i - 1] == letter and i - 1 not in been:
            grey += 1
            been.add(i - 1)
            continue
        if i < len(correct) - 1 and correct[i + 1] == letter and i + 1 not in been:
            grey += 1
            been.add(i + 1)
            continue

        for j, cl in enumerate(correct):
            if abs(i - j) >= 2 and j not in been and cl == letter:
                white += 1
                been.add(j)
                break

    print(f"{answer}: {black} black, {grey} grey, {white} white")


def main():
    while True:
        message = input()
        if message == "#":
            break

        tick(message)


if __name__ == "__main__":
    main()
