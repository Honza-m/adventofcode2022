import numpy as np
from pathlib import Path
from typing import List, Optional


def main():
    # Read file
    data = []
    filepath = Path(__file__).parent / "input.txt"
    with open(filepath, "r") as f:
        for line in f.readlines():
            data.append([int(x) for x in line.replace("\n", "")])

    # Process
    data = np.array(data)

    # Stars
    visible_trees = 0
    max_scenic_score = 0
    for x in range(1, data.shape[0] - 1):
        for y in range(1, data.shape[1] - 1):
            if is_visible(data, x, y):
                visible_trees += 1
            score = scenic_score(data, x, y)
            if score > max_scenic_score:
                max_scenic_score = score

    visible_outside = (data.shape[0] - 1) * 4
    print(f"There are {visible_trees + visible_outside} trees visible in total.")
    print(f"Max scenic score is {max_scenic_score}")


def is_visible(data, x, y):
    row = data[x, :]
    col = data[:, y]

    tree = data[x, y]

    left = row[:y].max()
    right = row[y + 1 :].max()
    top = col[:x].max()
    bottom = col[x + 1 :].max()

    if tree > left or tree > right or tree > top or tree > bottom:
        # print(f"{x}-{y} is visible")
        return True
    else:
        # print(f"{x}-{y} is not visible")
        return False


def scenic_score(data, x, y) -> int:
    row = data[x, :]
    col = data[:, y]

    main_tree = data[x, y]

    left = np.flip(row[:y])
    right = row[y + 1 :]
    top = np.flip(col[:x])
    bottom = col[x + 1 :]

    scores: List[int] = [0, 0, 0, 0]
    for s, line in enumerate([left, right, top, bottom]):
        for i, tree in enumerate(line):
            if tree >= main_tree:
                scores[s] = i + 1
                break
        if scores[s] == 0:
            scores[s] = len(line)

    return np.prod(scores, dtype=int)


if __name__ == "__main__":
    main()
