def main():
    chunbae = False
    nabi = False
    for _ in range(15):
        words = input().split()
        if "w" in words:
            chunbae = True
        elif "b" in words:
            nabi = True

    if chunbae:
        print("chunbae")
    elif nabi:
        print("nabi")
    else:
        print("yeongcheol")


if __name__ == "__main__":
    main()
