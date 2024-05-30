def main():
    _ = int(input())
    string = input()

    previous = ""
    result = 0
    used = False
    right = False
    vacant = True
    for letter in string:
        if letter == "S":
            if vacant:
                result += 1
                used = True
                vacant = True
            else:
                result += 1
                used = True
                vacant = False
        elif not right:
            if vacant:
                result += 1
                used = True
            else:
                used = False
                vacant = True
        else:
            used = True
            result += 1
            vacant = False

        if right:
            vacant = False

        if letter == "L":
            right = not right

        previous = letter

    if not used:
        result += 1

    print(result)


if __name__ == "__main__":
    main()
