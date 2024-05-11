prices = {
    "Poblano": 1500,
    "Mirasol": 6000,
    "Serrano": 15500,
    "Cayenne": 40000,
    "Thai": 75000,
    "Habanero": 125000,
}


def main():
    result = 0
    for _ in range(int(input())):
        result += prices[input()]

    print(result)


if __name__ == "__main__":
    main()
