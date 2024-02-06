def main():
    ssangs = set()
    while (word := input()) != '*':
        surprising = True
        for i in range(len(word) - 2):
            ssangs.clear()
            for j in range(len(word) - i - 1):
                ssang = word[j] + word[j+i+1]
                if ssang in ssangs:
                    surprising = False
                    break
                ssangs.add(ssang)
            if not surprising:
                break

        if surprising:
            print(f'{word} is surprising.')
        else:
            print(f'{word} is NOT surprising.')


if __name__ == '__main__':
    main()
