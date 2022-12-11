from collections import deque

NATO = (
    "ALFA", "BRAVO", "CHARLIE", "DELTA", "ECHO", "FOXTROT", "GOLF",
    "HOTEL", "INDIA", "JULIETT", "KILO", "LIMA", "MIKE", "NOVEMBER", "OSCAR", "PAPA",
    "QUEBEC", "ROMEO", "SIERRA", "TANGO", "UNIFORM", "VICTOR",
    "WHISKEY", "XRAY", "YANKEE", "ZULU"
)


def digest_queries(string, queries, max_length):
    print('\t', string, queries, max_length)
    count = queries.popleft()[1]

    for i in range(count):
        print(i)
        result = ''
        length = 0
        for letter in string:
            delta = NATO[ord(letter) - ord('A')]
            result += delta
            length += len(delta)

            if length >= max_length:
                break
        string = result

    while queries:
        arg = queries.popleft()[1]
        print(string[arg-1], end='')

    return string


def main():
    string, count = input().split()
    count = int(count)

    queries = deque([(1, 0)])
    max_length = 0
    for _ in range(count):
        action, arg = map(int, input().split())
        max_length = max(arg, max_length)

        if action == 1:
            string = digest_queries(string, queries, max_length)

        queries.append((action, arg))

    digest_queries(string, queries, max_length)


if __name__ == '__main__':
    main()
