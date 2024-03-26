def main():
    _, _, _ = map(int, input().split())

    missings = set(map(int, input().split()))
    extras = set(map(int, input().split()))

    for missing in sorted(missings):
        if missing in extras:
            extras.remove(missing)
            missings.remove(missing)
            continue

    for missing in sorted(missings):
        if missing - 1 in extras:
            extras.remove(missing - 1)
            missings.remove(missing)
            continue

        if missing + 1 in extras:
            extras.remove(missing + 1)
            missings.remove(missing)
            continue

    print(len(missings))


if __name__ == "__main__":
    main()
