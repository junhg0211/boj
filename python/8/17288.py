def main():
    message = input() + "$"

    previous = ""
    streak = 0
    result = 0
    for letter in message:
        # print(previous, letter, streak, result)
        if previous == chr(ord(letter) - 1):
            streak += 1
        else:
            if streak == 2:
                result += 1
            streak = 0

        previous = letter

    print(result)


if __name__ == "__main__":
    main()
