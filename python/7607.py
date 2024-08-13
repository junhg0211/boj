variables = dict()

OPERATORS = list("#+-*/=[]:") + ["print"]


def get_operator_priority(operator) -> int:
    if operator in ["[", "]"]:
        return 30
    if operator in ["*", "/"]:
        return 30
    if operator in ["+", "-"]:
        return 20
    if operator in [":"]:
        return 10
    return 0


def print_them(them, last=False):
    if isinstance(them, int):
        print(them, end="")
        return

    for i, value in enumerate(them):
        print_them(value, i == len(them) - 1)

    if not last:
        print(":", end="")


def handle_operator(operands, operators):
    operator = operators.pop()

    if operator == "print":
        print("OUT", end=" ")
        print(operands)
        print_them(operands, True)
        print()
        return

    operand1 = None
    if len(operands) >= 1:
        operand1 = operands.pop()

    operand2 = None
    if len(operands) >= 1:
        operand2 = operands.pop()

        if operator == ":":
            operands.append(operand2)
            operands.append(operand1)
            return


def treeify(tokens):
    operands = list()
    operators = list()

    for token in tokens:
        print(token, operands, operators)
        if isinstance(token, list) or token not in OPERATORS:
            operands.append(treeify(token))
        else:
            priority = get_operator_priority(token)
            while operators and get_operator_priority(operators[-1]) >= priority:
                handle_operator(operands, operators)
            operators.append(token)
    while operators:
        print("01", operands, operators)
        handle_operator(operands, operators)
    print("02", operands, operators)

    return operands


def tokenize(line):
    tokens = list()

    buffer = list()
    stack = 0
    for letter in line:
        buffer.append(letter)

        if stack:
            if letter == "(":
                stack += 1
            elif letter == ")":
                stack -= 1

            if stack == 0:
                buffer.pop()
                tokens.append(tokenize("".join(buffer)))
                buffer.clear()

        elif letter == "(":
            buffer.pop()
            stack += 1

        elif letter == " ":
            buffer.pop()
            tokens.append("".join(buffer))
            buffer.clear()

        elif letter in OPERATORS or letter in variables:
            operator = buffer.pop()
            if buffer:
                try:
                    tokens.append([int("".join(buffer))])
                except ValueError:
                    tokens.append("".join(buffer))
            tokens.append(operator)
            buffer.clear()

    if buffer:
        try:
            tokens.append([int("".join(buffer))])
        except ValueError:
            tokens.append("".join(buffer))

    return tokens


def main():
    while True:
        tokens = tokenize(input())
        print(0, tokens)

        if tokens[0] == "#":
            break

        print(1, treeify(tokens))


if __name__ == "__main__":
    main()
