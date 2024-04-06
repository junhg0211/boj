def main():
    message = input()

    result = message

    for i in range(1, len(message)):
        for j in range(i + 1, len(message)):
            this = message[:i][::-1] + message[i:j][::-1] + message[j:][::-1]

            if this < result:
                result = this

    print(result)


if __name__ == "__main__":
    main()
