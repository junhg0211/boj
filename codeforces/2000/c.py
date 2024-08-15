testcase_count = int(input())

for _ in range(testcase_count):
    template_length = int(input())
    template = list(map(int, input().split()))

    string_count = int(input())
    for string in (input() for _ in range(string_count)):
        mapping = dict()
        no = False
        used = set()
        if len(string) != template_length:
            no = True
        else:
            for j, letter in enumerate(string):
                if letter in mapping and mapping[letter] != template[j]:
                    no = True
                    break
                if letter not in mapping and template[j] in used:
                    no = True
                    break

                mapping[letter] = template[j]
                used.add(template[j])

        if no:
            print("NO")
        else:
            print("YES")
