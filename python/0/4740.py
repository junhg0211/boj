def main():
    while True:
        message = input()

        if message == "***":
            break

        print(message[::-1])


if __name__ == "__main__":
    main()
