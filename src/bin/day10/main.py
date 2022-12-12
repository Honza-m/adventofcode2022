from pathlib import Path


class Cycle:
    def __init__(self):
        self.count = 0
        self.register = 1

        # Star 1
        self.sample_cycles = [i for i in range(20, 221, 40)]
        self.samples = []

        # Star 2
        self.display = []

    def tick(self):
        # Star 2
        if self.count % 40 in [self.register - 1, self.register, self.register + 1]:
            self.display.append("#")
        else:
            self.display.append(".")

        # Tick
        self.count += 1

        # Star 1
        if self.count in self.sample_cycles:
            self.samples.append(self.register)

    def add_to_register(self, n):
        self.tick()
        self.tick()
        self.register += int(n)


def main():

    # Parse instructions
    filepath = Path(__file__).parent / "input.txt"
    with open(filepath, "r") as f:
        instructions = [x for x in f.read().splitlines()]

    # Run instructions
    cycle = Cycle()
    for ins in instructions:
        if ins == "noop":
            cycle.tick()
        elif ins.startswith("addx"):
            _, n = ins.split(" ")
            cycle.add_to_register(n)

    # Star 1 - Calculate signal strength
    res = 0
    for n, m in zip(cycle.sample_cycles, cycle.samples):
        res += n * m
    print(res)

    # Star 2
    for i in range(0, 239, 40):
        print("".join(cycle.display[i : i + 40]))


if __name__ == "__main__":
    main()
