def task1(fish, days=18):
    """
    find the location of the submarine after a given list of commands
    >>> task1([3, 4, 3, 1, 2])
    26
    """

    pop = [0 for _ in range(9)]

    for f in fish:
        pop[f] += 1

    for day in range(days):
        new_fish = pop[0]

        for idx, fish in enumerate(pop[1:]):
            pop[idx] = pop[idx + 1]

        pop[6] += new_fish
        pop[8] = new_fish

    return sum(pop)


if __name__ == "__main__":
    with open("inputs/day06.txt") as file:
        nums = [int(n) for n in file.read().split(",")]

        print("task1:", task1(nums, days=80))
        print("task2:", task1(nums, days=256))
