from sys import stdin, setrecursionlimit

input = stdin.readline

setrecursionlimit(10**6)


class Node:
    def __init__(self, value: int):
        self.value = value
        self.left = None
        self.right = None

    def add(self, node: 'Node'):
        if node.value < self.value:
            if self.left is None:
                self.left = node
            else:
                self.left.add(node)
        else:
            if self.right is None:
                self.right = node
            else:
                self.right.add(node)

    def root_last(self):
        self.left is not None and self.left.root_last()
        self.right is not None and self.right.root_last()
        print(self.value)


def main():
    root = Node(int(input()))

    while True:
        try:
            root.add(Node(int(input())))
        except EOFError:
            break
        except KeyboardInterrupt:
            break
        except ValueError:
            break

    root.root_last()


if __name__ == '__main__':
    main()
