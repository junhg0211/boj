def main():
    n = int(input())
    input()
    string = input()

    string += string[-1]

    sequence = list()
    streak = False
    length = 0
    previous = 'O'
    for i, letter in enumerate(string):
        if letter == 'I':
            if streak:
                if previous == 'O':
                    length += 1
                else:
                    if streak:
                        sequence.append(length)
                        length = 0
            else:
                if previous == 'O':
                    streak = True
        else:
            if previous == 'O':
                if streak:
                    streak = False
                    sequence.append(length)
                    length = 0

        previous = letter
        # print(f'{i=}, {letter=}, {streak=}, {previous=}, {sequence=}, {length=}')

    count = 0
    for length in sequence:
        count += max(length - n + 1, 0)

    print(count)

if __name__ == '__main__':
    main()
