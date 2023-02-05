def main():
    emoji = input()
    print(len(emoji) + emoji.count(':') + emoji.count('_')*5)


if __name__ == '__main__':
    main()
