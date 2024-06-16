def main():
    while True:
        word_count = int(input())

        if word_count == 0:
            break

        words = [input() for _ in range(word_count)]

        for j in range(word_count):
            good = True
            left = [7, 5, 7, 7]
            remains = 5
            for i in range(j, word_count):
                remains -= len(words[i])
                # print(words[i], remains)

                if remains == 0:
                    if not left:
                        break
                    remains = left.pop(0)
                elif remains < 0:
                    good = False
                    break

            if good:
                print(j + 1)
                break


if __name__ == "__main__":
    main()
