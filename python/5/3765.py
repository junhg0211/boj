def main():
    while True:
        try:
            print(input())
        except EOFError:
            break
        except KeyboardInterrupt:
            break


if __name__ == '__main__':
    main()
