codes = [
    "000000",
    "001111",
    "010011",
    "011100",
    "100110",
    "101001",
    "110101",
    "111010",
]


def main():
    count = int(input())
    string = input()

    buffer = ""

    for i in range(count):
        raw_letter = string[6 * i : 6 * i + 6]

        letter = ""
        for j, code in enumerate(codes):
            match = 0
            for k, letter in enumerate(code):
                if raw_letter[k] == letter:
                    match += 1
            if match >= 5:
                letter = chr(ord("A") + j)
                break

        if letter in ["0", "1"]:
            print(i + 1)
            return

        buffer += letter

    print(buffer)


if __name__ == "__main__":
    main()
