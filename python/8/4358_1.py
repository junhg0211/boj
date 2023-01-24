from sys import stdin

input = stdin.readline


class Tree:
    def __init__(self, value = None):
        self.value = value
        self.weight = 0

        self.left = None
        self.right = None

    def add(self, value: str):
        if self.value is None:
            self.value = value
            self.weight += 1
            return

        if value == self.value:
            self.weight += 1
            return

        if value < self.value:
            if self.left is None:
                self.left = Tree()
            self.left.add(value)
            return

        if self.right is None:
            self.right = Tree()
        self.right.add(value)

    def iterate(self, count: int):
        self.left is not None and self.left.iterate(count)
        print(f'{self.value} {self.weight/count * 100:.4f}')
        self.right is not None and self.right.iterate(count)


def main():
    root = Tree()
    total_count = 0

    while True:
        try:
            name = input().rstrip()
        except EOFError:
            break
        except KeyboardInterrupt:
            break
        if not name:
            break

        root.add(name)
        total_count += 1

    root.iterate(total_count)


if __name__ == '__main__':
    main()
