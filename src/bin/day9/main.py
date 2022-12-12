from dataclasses import dataclass, field
from pathlib import Path
from typing import Set, List
from pprint import pprint


@dataclass
class Coordinates:
    x: int
    y: int

    def __str__(self):
        return f"{self.x}|{self.y}"


@dataclass
class Snake:
    number: int
    head: Coordinates
    tail: Coordinates
    tail_visited: Set[str] = field(default_factory=set)

    @classmethod
    def from_empty(cls, n):
        return cls(n, Coordinates(0, 0), Coordinates(0, 0))

    @classmethod
    def from_coor(cls, n, head_x, head_y, tail_x, tail_y):
        return cls(n, Coordinates(head_x, head_y), Coordinates(tail_x, tail_y))

    def move_head(self, direction):
        # Move head
        if direction == "U":
            self.head.y += 1
        elif direction == "D":
            self.head.y -= 1
        elif direction == "R":
            self.head.x += 1
        elif direction == "L":
            self.head.x -= 1
        else:
            raise ValueError("Unrecognised direction")

        self.pull_tail()

    def pull_tail(self):
        # How much further is tail from head?
        x_diff = abs(self.head.x - self.tail.x)
        y_diff = abs(self.head.y - self.tail.y)

        # Return if not too far
        if x_diff <= 1 and y_diff <= 1:
            self.tail_visited.add(str(self.tail))
            return

        if x_diff == y_diff:
            # Move diagonally towards head
            if self.head.x > self.tail.x:
                self.tail.x += 1
            else:
                self.tail.x -= 1
            if self.head.y > self.tail.y:
                self.tail.y += 1
            else:
                self.tail.y -= 1
        elif x_diff > y_diff:
            # Move x towards head and set y to be the same
            self.tail.y = self.head.y
            if self.head.x > self.tail.x:
                self.tail.x += 1
            else:
                self.tail.x -= 1
        elif y_diff > x_diff:
            # Move y towards head and set x to be the same
            self.tail.x = self.head.x
            if self.head.y > self.tail.y:
                self.tail.y += 1
            else:
                self.tail.y -= 1

        # Mark tail position
        self.tail_visited.add(str(self.tail))

    def __repr__(self):
        return "Snake position - {} - {}".format(self.head, self.tail)


def print_map():
    x = [s.head.x for s in SNAKES]
    y = [s.head.y for s in SNAKES]
    # Include 0,0 on the map
    x.append(0)
    y.append(0)

    rows = []
    for r in range(min(y), max(y) + 1):
        row = []

        for c in range(min(x), max(x) + 1):
            cell = "."
            if r == 0 and c == 0:
                cell = "s"
            for snake in SNAKES:
                if snake.head.x == c and snake.head.y == r:
                    cell = str(snake.number)
                    break
            row.append(cell)

        rows.insert(0, row)

    pprint(rows)
    print()


SNAKES = []


def main():
    filepath = Path(__file__).parent / "input.txt"
    with open(filepath, "r") as f:
        lines = f.readlines()

    snake_total = 9
    for s in range(snake_total):
        SNAKES.append(Snake.from_empty(s))

    for line in lines:
        direction, n = line.replace("\n", "").split(" ")
        for _ in range(int(n)):
            SNAKES[0].move_head(direction)
            for i in range(1, snake_total):
                SNAKES[i].head = Coordinates(SNAKES[i - 1].tail.x, SNAKES[i - 1].tail.y)
                SNAKES[i].pull_tail()
            # print_map()

    print(len(SNAKES[-1].tail_visited))


if __name__ == "__main__":
    main()
