def task1(crabs):
    """
    find the location of the submarine after a given list of commands
    >>> task1([16, 1, 2, 0, 4, 2, 7, 1, 2, 14])
    37
    """

    def fuel_cost(crab_location):
        return sum([abs(crab_location - crab) for crab in crabs])

    return min([fuel_cost(crab) for crab in crabs])


def task2(crabs):
    """
    find the location of the submarine after a given list of commands
    >>> task2([16, 1, 2, 0, 4, 2, 7, 1, 2, 14])
    168
    """

    def fuel_cost(crab_location):
        return sum([gauss_dist(crab, crab_location) for crab in crabs])

    def gauss_dist(c1, c2):
        dist = abs(c1 - c2 - 1)
        return int((dist * (dist + 1)) / 2)

    return min([fuel_cost(crab) for crab in crabs])


if __name__ == "__main__":
    with open("inputs/day07.txt") as file:
        nums = [int(n) for n in file.read().split(",")]

        print("task1:", task1(nums))
        print("task2:", task2(nums))
