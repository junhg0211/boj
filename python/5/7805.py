def main():
    while True:
        try:
            string = list(input())
        except EOFError:
            break

        swaps = 0
        previous = '0'
        for i in range(len(string)):
            index = -1
            now = 'z'
            for j in range(i, len(string)):
                if string[j] >= previous and string[j] <= now:
                    index = j
                    now = string[j]

            if index == i:
                continue

            string[index], string[i] = string[i], string[index]
            swaps += 1
            previous = string[i]

        print(swaps)


if __name__ == '__main__':
    main()
