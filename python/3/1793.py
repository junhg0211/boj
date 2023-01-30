cache = [1, 1]


def tick(value: int):
    while len(cache) <= value:
        cache.append(cache[-2]*2 + cache[-1])

    print(cache[value])


def main():
    while True:
        try:
            value = int(input())
        except EOFError:
            break
        except ValueError:
            break
        except KeyboardInterrupt:
            break

        tick(value)


if __name__ == '__main__':
    main()
