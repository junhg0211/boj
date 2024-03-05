import re

WORD_E = r"[a-zA-z0-9\$\?\+\_]+"
OPERATOR_E = r"<|>|&&|\|\||\(|\)"
TOKEN_E = r"({})|({})".format(WORD_E, OPERATOR_E)
TOKEN = re.compile(TOKEN_E)


def main():
    message = input()

    for token in TOKEN.finditer(message):
        a, b = token.span()
        print(message[a:b], end=" ")
    print()


if __name__ == "__main__":
    main()
