from sys import stdin

input = stdin.readline


def main():
    count = int(input())
    words = sorted((input().rstrip() for _ in range(count)), key=len, reverse=True)

    i = 0
    result = 0
    result_set = set()
    while i < count-result:
        result_set.clear()

        for j in range(i, count):
            word = words[j]
            ok = True
            for wordis in result_set:
                if wordis.startswith(word):
                    ok = False
                    break
            if not ok:
                continue
            result_set.add(word)

        # print(result_set)
        result = max(result, len(result_set))

        i += 1

    print(result)


if __name__ == '__main__':
    main()
